use std::collections::BTreeMap;
use std::convert::{From, Into};
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;

use atomicwrites::{AtomicFile, OverwriteBehavior};
use serde_json as json;

use crate::error::Error;
use crate::store::Store;
use crate::value::Value;

pub struct Database {
    store: Store,
    path: Vec<String>
}

impl Database {
    pub fn new (store: Store) -> Self {
        Database {
            store,
            path: vec![],
        }
    }

    pub fn get (&self, path: Vec<String>) -> Result<Value, Error> {
        let traversed = traverse_tree(&path, &self.store);
        if traversed.is_none() { return Err(Error::NotFound) }
        let (extra_path, store) = traversed.unwrap();

        let depth = path.len() - extra_path.len();
        let value = get_in_store(store, &path, depth)?;

        Ok(value)
    }

    pub fn walk (&self, path: Vec<String>) -> Option<Database> {
        match traverse_tree(&path, &self.store) {
            None => None,
            Some((extra_path, store)) => {
                let depth = path.len() - extra_path.len();
                let nested_path = path.get(0..depth).unwrap().to_vec();

                Some(Database {
                    store: (*store).clone(),
                    path: nested_path,
                })
            }
        }
    }

    pub fn set (&self, path: Vec<String>, value: &Value) -> Result<(), Error> {
        let traversed = traverse_tree(&path, &self.store);
        if traversed.is_none() { return Err(Error::NotFound) }
        let (extra_path, store) = traversed.unwrap();

        let depth = path.len() - extra_path.len();
        set_in_store(store, &path, value, depth)
    }
}

fn read_file (path: String) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    Ok(data)
}

fn write_file (path: String, data: String) -> Result<(), io::Error> {
    let atomic_file = AtomicFile::new(path, OverwriteBehavior::AllowOverwrite);
    match atomic_file.write(|file| file.write_all(data.as_bytes())) {
        Ok(()) => Ok(()),
        Err(atomicwrites::Error::Internal(io_error)) => Err(io_error),
        Err(atomicwrites::Error::User(io_error)) => Err(io_error),
    }
}

fn traverse_tree <'a, 'b> (path: &'a [String], store: &'b Store) -> Option<(&'a [String], &'b Store)> {
    match store {
        Store::Tree(map) => {
            if path.len() == 0 { return Some((path, store)) }
            let key = path.get(0).unwrap();
            let next_path = path.get(1..path.len()).unwrap();
            match map.get(key) {
                Some(next_store) => traverse_tree(next_path, next_store),
                None => None
            }
        },
        leaf => Some((path, leaf))
    }
}

fn get_in_store (store: &Store, path: &[String], depth: usize) -> Result<Value, Error> {
    if let Store::Tree(map) = store {
        let mut next_map = BTreeMap::new();
        map.into_iter().try_for_each(|(key, store)| -> Result<(), Error> {
            let value = get_in_store(store, path, depth + 1)?;
            next_map.insert(key.clone(), value);
            Ok(())
        })?;
        return Ok(Value::Object(next_map));
    }

    let file_path = path.get(0..depth).unwrap();
    let file_extension = store_file_extension(store)?;
    let file_path_string: String = String::from("./") + &file_path.join("/") + &file_extension;

    let data = read_file(file_path_string)?;
    let value = store_data_to_value(store, &data)?;

    let value_path = path.get(depth..path.len()).unwrap();
    get_in_value(value_path, value)
}

fn set_in_store (store: &Store, path: &[String], value: &Value, depth: usize) -> Result<(), Error> {
    if let Store::Tree(map) = store {
        return map.into_iter().try_for_each(|(key, store)| -> Result<(), Error> {
            if let Value::Object(object) = value {
                if let Some(nested_value) = object.get(key) {
                    set_in_store(store, path, nested_value, depth + 1)?;
                }
                Ok(())
            } else {
                Err(Error::BadInput)
            }
        })
    }

    let file_path = path.get(0..depth).unwrap();
    let file_extension = store_file_extension(store)?;
    let file_path_string: String = String::from("./") + &file_path.join("/") + &file_extension;

    let data = read_file(file_path_string.clone())?;
    let file_value = store_data_to_value(store, &data)?;

    let value_path = path.get(depth..path.len()).unwrap();
    let next_file_value = set_in_value(file_value, value_path, value.clone())?;

    let data = store_value_to_data(store, &next_file_value)?;
    write_file(file_path_string.clone(), data)?;

    Ok(())
}

fn get_in_value (path: &[String], value: Value) -> Result<Value, Error> {
    if path.len() == 0 { return Ok(value); }
    match value {
        Value::Object(object) => {
            let key = path.get(0).unwrap();
            let next_path = path.get(1..path.len()).unwrap();
            let next_value = object.get(key).ok_or(Error::NotFound)?;
            get_in_value(next_path, next_value.clone())
        },
        _ => Ok(value)
    }
}

fn set_in_value (value: Value, path: &[String], next_value_at_path: Value) -> Result<Value, Error> {
    if path.len() == 0 { return Ok(next_value_at_path); }
    match value {
        Value::Object(map) => {
            let next_key = path.get(0).unwrap().clone();
            let next_path = path.get(1..path.len()).unwrap();
            let mut next_map = BTreeMap::new();
            map.into_iter().try_for_each(|(key, nested_value)| -> Result<(), Error> {
                let next_nested_value = if key == next_key {
                    set_in_value(nested_value, next_path, next_value_at_path.clone())?
                } else {
                    nested_value
                };
                next_map.insert(key.clone(), next_nested_value);
                Ok(())
            })?;
            Ok(Value::Object(next_map))
        },
        _ => Ok(value)
    }
}

fn store_file_extension (store: &Store) -> Result<String, Error> {
    match store {
        Store::Json => Ok(".json".into()),
        _ => return Err(Error::Unexpected)
    }
}

fn store_data_to_value (store: &Store, data: &String) -> Result<Value, Error> {
    match store {
        Store::Json => {
            let json_value: json::Value = json::from_str(&data)?;
            Ok(json_value.into())
        },
        _ => return Err(Error::Unexpected)
    }
}

fn store_value_to_data (store: &Store, value: &Value) -> Result<String, Error> {
    match store {
        Store::Json => {
            let json_value: json::Value = value.clone().into();
            let json_string = json::to_string_pretty(&json_value)?;
            Ok(json_string)
        },
        _ => return Err(Error::Unexpected)
    }
}
