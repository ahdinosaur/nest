use std::collections::BTreeMap;
use std::convert::Into;
use std::io;
use std::path::{Path, PathBuf, MAIN_SEPARATOR};

use log::{debug, info};
use mkdirp::mkdirp;

use crate::error::Error;
use crate::schema::Schema;
use crate::value::Value;

/// The entry point for a Nest data store.
///
/// Stores start with a *root* path and use a *schema* to map the topology of your data structures
/// with the filesystem (files and directories).
///
/// # Example
///
/// Given a filesystem setup like:
///
/// ```txt
/// $ tree /home/dinosaur/example
/// /home/dinosaur/example
/// └── hello
///     └── world.json
///
/// $ cat /home/dinosaur/example/hello/world.json
/// {
///   "nest": "🐣"
/// }
/// ```
///
/// We can create a simple Nest data store with:
///
/// ```rust, no_run
/// use serde_json::json;
/// use nest::{Store, Value};
///
/// let root = "/home/dinosaur/example";
/// let schema = json!({
///     "hello": {
///         "world": "json"
///     }
/// }).into();
/// let store = Store::new(root, schema);
/// ```
///
/// Now we can use this store to get and set values.
///
/// ```rust, no_run
/// # use nest::{Store, Error, Value};
/// # let store = Store::new("./", serde_json::json!({}).into());
/// let value = store.get(&["hello", "world", "nest"])?;
/// assert_eq!(value, Value::String("🐣".into()));
///
/// let next_value = Value::String("🐥".into());
/// store.set(&["hello", "world", "nest"], &next_value)?;
/// # Ok::<(), Error>(())
/// ```
///
/// If we mostly care about data starting with a given path within the Nest, we can create a
/// sub-Store that contains our path as a new root.
///
/// ```rust, no_run
/// # use nest::{Store, Error, Value};
/// # let store = Store::new("./", serde_json::json!({}).into());
/// let sub = store.sub(&["hello", "world"])?;
///
/// let value = store.get(&["nest"])?;
/// assert_eq!(value, Value::String("🐥".into()));
///
/// let next_value = Value::String("🐔".into());
/// store.set(&["nest"], &next_value)?;
/// # Ok::<(), Error>(())
/// ```
///

pub struct Store {
    root: PathBuf,
    schema: Schema,
}

impl Store {
    /// Create a `Store` from `root` path and `schema` mapping.
    pub fn new<A>(root: A, schema: Schema) -> Self
    where
        A: Into<PathBuf>,
    {
        let root = root.into();
        info!("nest::Store::new({:?}, {:?})", root, schema);

        // TODO validate schema
        Store { root, schema }
    }

    /// Get the `Value` at the given `path`.
    pub fn get(&self, path: &[&str]) -> Result<Value, Error> {
        info!("nest::Store#get({:?})", path);

        let traversed = traverse_schema(path, &self.schema);
        if traversed.is_none() {
            return Err(Error::NotFoundInSchema);
        }
        let (extra_path, schema) = traversed.unwrap();

        debug!("extra_path: {:?}", extra_path);

        let depth = path.len() - extra_path.len();
        let value = get_in_schema(schema, &self.root, path, depth)?;

        Ok(value)
    }

    /// Set the `Value` at the given `path`.
    pub fn set(&self, path: &[&str], value: &Value) -> Result<(), Error> {
        info!("nest::Store#set({:?}), {:?}", path, value);

        let traversed = traverse_schema(path, &self.schema);
        if traversed.is_none() {
            return Err(Error::NotFoundInSchema);
        }
        let (extra_path, schema) = traversed.unwrap();

        let depth = path.len() - extra_path.len();
        set_in_schema(schema, &self.root, path, value, depth)
    }

    /// Return a sub-`Store` at the given `path`.
    pub fn sub(&self, path: &[&str]) -> Result<Store, Error> {
        match traverse_schema(path, &self.schema) {
            None => Err(Error::NotFoundInSchema),
            Some((extra_path, schema)) => {
                let depth = path.len() - extra_path.len();
                let nested_path = &path[0..depth].to_vec();

                Ok(Store {
                    schema: (*schema).clone(),
                    root: self
                        .root
                        .join(nested_path.join(&MAIN_SEPARATOR.to_string())),
                })
            }
        }
    }
}

fn traverse_schema<'a, 'b, 'c>(
    path: &'a [&'b str],
    schema: &'c Schema,
) -> Option<(&'a [&'b str], &'c Schema)> {
    match schema {
        Schema::Directory(map) => {
            if path.is_empty() {
                return Some((path, schema));
            }
            let key = path[0];
            let next_path = &path[1..path.len()];
            match map.get(key) {
                Some(next_schema) => traverse_schema(next_path, next_schema),
                None => None,
            }
        }
        leaf => Some((path, leaf)),
    }
}

fn get_in_schema(
    schema: &Schema,
    root: &Path,
    path: &[&str],
    depth: usize,
) -> Result<Value, Error> {
    debug!(
        "get_in_schema({:?}, {:?}, {:?}, {:?})",
        schema, root, path, depth
    );

    match schema {
        // if schema is a directory, it refers to a nested value
        Schema::Directory(map) => {
            let mut next_map = BTreeMap::new();
            map.iter()
                .try_for_each(|(key, nested_schema)| -> Result<(), Error> {
                    let nested_path = {
                        let mut vec = Vec::new();
                        vec.extend(path.iter().cloned());
                        vec.push(key);
                        vec
                    };
                    let value = get_in_schema(nested_schema, root, &nested_path, depth + 1)?;
                    next_map.insert(key.clone(), value);
                    Ok(())
                })?;
            Ok(Value::Object(next_map))
        }
        Schema::Source(source) => {
            // otherwise schema is a source (file)
            let source_path: PathBuf = root.join(path[0..depth].join(&MAIN_SEPARATOR.to_string()));

            // read the file as a value
            let source_value = source.read(&source_path)?;

            // get value within source (file) value at path
            let value_path = &path[depth..path.len()];
            get_in_value(value_path, source_value)
        }
    }
}

fn set_in_schema(
    schema: &Schema,
    root: &Path,
    path: &[&str],
    value: &Value,
    depth: usize,
) -> Result<(), Error> {
    match schema {
        // if schema is a directory, it refers to a nested value
        Schema::Directory(map) => {
            if let Value::Object(object) = value {
                return map
                    .iter()
                    .try_for_each(|(key, nested_schema)| -> Result<(), Error> {
                        let nested_path = {
                            let mut vec = Vec::new();
                            vec.extend(path.iter().cloned());
                            vec.push(key);
                            vec
                        };
                        if let Some(nested_value) = object.get(key) {
                            set_in_schema(
                                nested_schema,
                                root,
                                &nested_path,
                                nested_value,
                                depth + 1,
                            )?;
                        }
                        Ok(())
                    });
            } else {
                Err(Error::ExpectedObjectValueForDirectorySchema)
            }
        }
        // otherwise schema is a source (file)
        Schema::Source(source) => {
            let source_path: PathBuf = root.join(path[0..depth].join(&MAIN_SEPARATOR.to_string()));

            // ensure parent directory exists
            mkdirp(&source_path.parent().unwrap())?;

            let source_value = match source.read(&source_path) {
                Err(Error::Io(err)) => {
                    match err.kind() {
                        io::ErrorKind::NotFound => {
                            // otherwise default to an empty object
                            Ok(Value::Object(BTreeMap::new()))
                        }
                        _ => Err(Error::Io(err)),
                    }
                }
                result => result,
            }?;

            // set value at path
            let value_path = &path[depth..path.len()];
            let next_value = set_in_value(source_value, value_path, value.clone())?;

            // write new value to source (file)
            source.write(&source_path, &next_value)?;

            Ok(())
        }
    }
}

fn get_in_value(path: &[&str], value: Value) -> Result<Value, Error> {
    if path.is_empty() {
        return Ok(value);
    }
    match value {
        Value::Object(object) => {
            let key = path[0];
            let next_path = &path[1..path.len()];
            let next_value = object.get(key).ok_or(Error::NotFoundInValue)?;
            get_in_value(next_path, next_value.clone())
        }
        _ => Ok(value),
    }
}

fn set_in_value(value: Value, path: &[&str], next_value_at_path: Value) -> Result<Value, Error> {
    if path.is_empty() {
        return Ok(next_value_at_path);
    }

    match value {
        Value::Object(map) => {
            let next_key = path[0].to_string();
            let mut next_map = map.clone();
            next_map.insert(next_key, next_value_at_path);
            Ok(Value::Object(next_map))
        }
        _ => set_in_value(Value::Object(BTreeMap::new()), path, next_value_at_path),
    }
}
