use std::convert::{From, Into};
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use std::iter::FromIterator;
use std::option;

use atomicwrites::{AtomicFile, OverwriteBehavior};
use common_macros::b_tree_map;
use serde_json as json;
use serde_json::Number;

fn main() {
    let store = Store::Tree(b_tree_map! {
        "test".into() => Store::Tree(b_tree_map! {
            "foo".into() => Store::Json
        })
    });
    let database = Database::new(
        store
    );
    let foobar = database.get(vec!["test".into(), "foo".into(), "bar".into()]).unwrap();
    println!("foobar {:?}", foobar);

    database.set(vec!["test".into(), "foo".into(), "bar".into()], &Value::String("hello".into())).unwrap();

    let ab = database.get(vec!["a".into(), "b".into()]);
    println!("ab {:?}", ab);
}

struct Database {
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

    pub fn get (&self, path: Vec<String>) -> Result<Value, DatabaseError> {
        let traversed = traverse_tree(&path, &self.store);
        if traversed.is_none() { return Err(DatabaseError::NotFound) }
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

    pub fn set (&self, path: Vec<String>, value: &Value) -> Result<(), DatabaseError> {
        let traversed = traverse_tree(&path, &self.store);
        if traversed.is_none() { return Err(DatabaseError::NotFound) }
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
    let mut atomic_file = AtomicFile::new(path, OverwriteBehavior::AllowOverwrite);
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

fn get_in_store (store: &Store, path: &[String], depth: usize) -> Result<Value, DatabaseError> {
    if let Store::Tree(map) = store {
        let mut next_map = BTreeMap::new();
        map.into_iter().try_for_each(|(key, store)| -> Result<(), DatabaseError> {
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

fn set_in_store (store: &Store, path: &[String], value: &Value, depth: usize) -> Result<(), DatabaseError> {
    if let Store::Tree(map) = store {
        return map.into_iter().try_for_each(|(key, store)| -> Result<(), DatabaseError> {
            if let Value::Object(object) = value {
                if let Some(nested_value) = object.get(key) {
                    set_in_store(store, path, nested_value, depth + 1)?;
                }
                Ok(())
            } else {
                Err(DatabaseError::BadInput)
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

fn get_in_value (path: &[String], value: Value) -> Result<Value, DatabaseError> {
    if path.len() == 0 { return Ok(value); }
    match value {
        Value::Object(object) => {
            let key = path.get(0).unwrap();
            let next_path = path.get(1..path.len()).unwrap();
            let next_value = object.get(key).ok_or(DatabaseError::NotFound)?;
            get_in_value(next_path, next_value.clone())
        },
        _ => Ok(value)
    }
}

fn set_in_value (value: Value, path: &[String], next_value_at_path: Value) -> Result<Value, DatabaseError> {
    if path.len() == 0 { return Ok(next_value_at_path); }
    match value {
        Value::Object(map) => {
            let next_key = path.get(0).unwrap().clone();
            let next_path = path.get(1..path.len()).unwrap();
            let mut next_map = BTreeMap::new();
            map.into_iter().try_for_each(|(key, nested_value)| -> Result<(), DatabaseError> {
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

fn store_file_extension (store: &Store) -> Result<String, DatabaseError> {
    match store {
        Store::Json => Ok(".json".into()),
        _ => return Err(DatabaseError::Unexpected)
    }
}

fn store_data_to_value (store: &Store, data: &String) -> Result<Value, DatabaseError> {
    match store {
        Store::Json => {
            let json_value: json::Value = json::from_str(&data)?;
            Ok(json_value.into())
        },
        _ => return Err(DatabaseError::Unexpected)
    }
}

fn store_value_to_data (store: &Store, value: &Value) -> Result<String, DatabaseError> {
    match store {
        Store::Json => {
            let json_value: json::Value = value.clone().into();
            let json_string = json::to_string_pretty(&json_value)?;
            Ok(json_string)
        },
        _ => return Err(DatabaseError::Unexpected)
    }
}

#[derive(Debug, Clone)]
enum Store {
    Tree(BTreeMap<String, Store>),
    Json
}

#[derive(Debug, Clone)]
enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Object(BTreeMap<String, Value>),
}

impl From<json::Value> for Value {
    fn from(value: json::Value) -> Value {
        match value {
            json::Value::Null => Value::Null,
            json::Value::Bool(bool) => Value::Bool(bool),
            json::Value::Number(number) => Value::Number(number),
            json::Value::String(string) => Value::String(string),
            json::Value::Array(array) => Value::Array(Vec::from_iter(array.into_iter().map(Self::from))),
            json::Value::Object(object) => Value::Object(BTreeMap::from_iter(object.into_iter().map(|(key, value)| (key, Self::from(value)))))
        }
    }
}

impl From<Value> for json::Value {
    fn from(value: Value) -> json::Value {
        match value {
            Value::Null => json::Value::Null,
            Value::Bool(bool) => json::Value::Bool(bool),
            Value::Number(number) => json::Value::Number(number),
            Value::String(string) => json::Value::String(string),
            Value::Array(array) => json::Value::Array(Vec::from_iter(array.into_iter().map(Self::from))),
            Value::Object(object) => json::Value::Object(json::map::Map::from_iter(object.into_iter().map(|(key, value)| (key, Self::from(value)))))
        }
    }
}


#[derive(Debug)]
enum DatabaseError {
    Io(io::Error),
    Json(json::error::Error),
    NotFound,
    BadInput,
    Unexpected
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DatabaseError::Io(ref err) => err.fmt(f),
            DatabaseError::Json(ref err) => err.fmt(f),
            DatabaseError::NotFound => write!(f, "Path not found"),
            DatabaseError::BadInput => write!(f, "Bad input"),
            DatabaseError::Unexpected => write!(f, "Unexpected (programmer) error"),
        }
    }
}

impl Error for DatabaseError {
    fn description(&self) -> &str {
        match *self {
            DatabaseError::Io(ref err) => err.description(),
            DatabaseError::Json(ref err) => err.description(),
            DatabaseError::NotFound => "not found",
            DatabaseError::Unexpected => "unexpected",
            DatabaseError::BadInput => "bad input",
        }
    }
}

impl From<io::Error> for DatabaseError {
    fn from(err: io::Error) -> DatabaseError {
        DatabaseError::Io(err)
    }
}

impl From<json::error::Error> for DatabaseError {
    fn from(err: json::error::Error) -> DatabaseError {
        DatabaseError::Json(err)
    }
}

