use std::io;
use std::path;

use indexmap::IndexMap;
use log::{debug, info};
use mkdirp::mkdirp;
use snafu::{ensure, OptionExt, ResultExt};

use crate::error::{self, Error, Result};
use crate::path::Path;
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
/// use std::convert::TryInto;
/// use serde_json::json;
/// use nest::{Error, Store, Value};
///
/// let root = "/home/dinosaur/example";
/// let schema = json!({
///     "hello": {
///         "world": "json"
///     }
/// }).try_into()?;
/// let store = Store::new(root, schema);
/// # Ok::<(), Error>(())
/// ```
///
/// Now we can use this store to get and set values.
///
/// ```rust, no_run
/// # use std::convert::TryInto;
/// # use nest::{Store, Error, Value};
/// # let store = Store::new("./", serde_json::json!({}).try_into()?);
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
/// # use std::convert::TryInto;
/// # use nest::{Store, Error, Value};
/// # let store = Store::new("./", serde_json::json!({}).try_into()?);
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
    root: path::PathBuf,
    schema: Schema,
}

impl Store {
    /// Create a `Store` from `root` path and `schema` mapping.
    pub fn new<A>(root: A, schema: Schema) -> Self
    where
        A: Into<path::PathBuf>,
    {
        let root = root.into();
        info!("nest::Store::new({:?}, {:?})", root, schema);

        // TODO validate schema
        Store { root, schema }
    }

    /// Get the `Value` at the given `path`.
    pub fn get<A>(&self, path: A) -> Result<Value>
    where
        A: Into<Path>,
    {
        let path = path.into();
        info!("nest::Store#get({:?})", path);

        let (extra_path, schema) = traverse_schema(path.clone(), &self.schema)
            .context(error::GetSchema { path: path.clone() })?;

        debug!("extra_path: {:?}", extra_path);

        let depth = path.len() - extra_path.len();
        let value = get_in_schema(schema, &self.root, path.clone(), depth)?;

        Ok(value)
    }

    /// Set the `Value` at the given `path`.
    pub fn set<A>(&self, path: A, value: &Value) -> Result<()>
    where
        A: Into<Path>,
    {
        let path = path.into();
        info!("nest::Store#set({:?}), {:?}", path, value);

        let (extra_path, schema) = traverse_schema(path.clone(), &self.schema)
            .context(error::GetSchema { path: path.clone() })?;

        let depth = path.clone().len() - extra_path.len();
        set_in_schema(schema, &self.root, path.clone(), value, depth)
    }

    /// Return a sub-`Store` at the given `path`.
    pub fn sub<A>(&self, path: A) -> Result<Store>
    where
        A: Into<Path>,
    {
        let path = path.into();

        let (extra_path, schema) = traverse_schema(path.clone(), &self.schema)
            .context(error::GetSchema { path: path.clone() })?;

        let depth = path.len() - extra_path.len();
        let nested_path = path.take(depth);

        Ok(Store {
            schema: (*schema).clone(),
            root: self.root.join(nested_path.to_path()),
        })
    }
}

fn traverse_schema(path: Path, schema: &Schema) -> Option<(Path, &Schema)> {
    match schema {
        Schema::Directory(map) => {
            if path.is_empty() {
                return Some((path, schema));
            }
            let key = path.first();
            let next_path = path.rest();
            match map.get(key) {
                Some(next_schema) => traverse_schema(next_path, next_schema),
                None => None,
            }
        }
        leaf => Some((path, leaf)),
    }
}

fn get_in_schema(schema: &Schema, root: &path::Path, path: Path, depth: usize) -> Result<Value> {
    debug!(
        "get_in_schema({:?}, {:?}, {:?}, {:?})",
        schema, root, path, depth
    );

    match schema {
        // if schema is a directory, it refers to a nested value
        Schema::Directory(map) => {
            let mut next_map = IndexMap::new();
            map.iter()
                .try_for_each(|(key, nested_schema)| -> Result<()> {
                    let nested_path = path.append(&key);
                    let value = get_in_schema(nested_schema, root, nested_path, depth + 1)?;
                    next_map.insert(key.clone(), value);
                    Ok(())
                })?;
            Ok(Value::Object(next_map))
        }
        Schema::Source(source) => {
            // otherwise schema is a source (file)
            let source_path: path::PathBuf = root.join(path.take(depth).to_path());

            // read the file as a value
            let source_value = source.read(source_path)?;

            // get value within source (file) value at path
            let value_path = path.skip(depth);
            get_in_value(value_path, source_value)
        }
    }
}

fn set_in_schema(
    schema: &Schema,
    root: &path::Path,
    path: Path,
    value: &Value,
    depth: usize,
) -> Result<()> {
    match schema {
        // if schema is a directory, it refers to a nested value
        Schema::Directory(map) => {
            ensure!(
                value.is_object(),
                error::SetObjectValueWhenDirectory { path: path.clone() }
            );

            let object = value.as_object().unwrap();

            map.iter()
                .try_for_each(|(key, nested_schema)| -> Result<()> {
                    let nested_path = path.append(key);
                    if let Some(nested_value) = object.get(key) {
                        set_in_schema(nested_schema, root, nested_path, nested_value, depth + 1)?;
                    }
                    Ok(())
                })
        }
        // otherwise schema is a source (file)
        Schema::Source(source) => {
            let source_path: path::PathBuf = root.join(path.take(depth).to_path());

            // ensure parent directory exists
            let directory_path = source_path.parent().unwrap();
            mkdirp(&directory_path).context(error::MakeDirectory {
                path: directory_path,
            })?;

            let source_value = match source.read(source_path.clone()) {
                Err(err) => {
                    if let Error::ReadSource { ref source, .. } = err {
                        match source.kind() {
                            io::ErrorKind::NotFound => {
                                // otherwise default to an empty object
                                Ok(Value::Object(IndexMap::new()))
                            }
                            _ => Err(err),
                        }
                    } else {
                        Err(err)
                    }
                }
                result => result,
            }?;

            // set value at path
            let value_path = path.skip(depth);
            let next_value = set_in_value(source_value, value_path, value.clone())?;

            // write new value to source (file)
            source.write(source_path.clone(), &next_value)?;

            Ok(())
        }
    }
}

fn get_in_value(path: Path, value: Value) -> Result<Value> {
    if path.is_empty() {
        return Ok(value);
    }
    match value {
        Value::Object(object) => {
            let key = path.first();
            let next_path = path.rest();
            let next_value = object
                .get(key)
                .context(error::GetSchema { path: path.clone() })?;
            get_in_value(next_path, next_value.clone())
        }
        _ => Ok(value),
    }
}

fn set_in_value(value: Value, path: Path, next_value_at_path: Value) -> Result<Value> {
    if path.is_empty() {
        return Ok(next_value_at_path);
    }

    match value {
        Value::Object(map) => {
            let next_key = path.first().to_string();
            let mut next_map = map.clone();
            next_map.insert(next_key, next_value_at_path);
            Ok(Value::Object(next_map))
        }
        _ => set_in_value(Value::Object(IndexMap::new()), path, next_value_at_path),
    }
}
