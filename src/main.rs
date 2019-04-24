use std::convert::{From, Into};
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::Read;
use std::iter::FromIterator;
use std::option;

use common_macros::b_tree_map;
use serde_json as json;
use serde_json::Number;

trait Store {
    fn get (&self, path: &[String]) -> Result<Value, StoreError>;
    fn set (&self, path: &[String], value: Value) -> Result<(), StoreError>;
}

struct Tree {
    pub name: String,
    pub parent: Option<Box<Tree>>,
    pub childen: Vec<Box<Store>>,
}

impl Tree {
    fn path (&self) -> Vec<String> {
        let path = Vec::new();
        path.push(self.name);
        let mut parent_option = self.parent;
        while parent_option.is_some() {
            let parent = parent_option.unwrap();
            path.insert(0, parent.name);
            parent_option = parent.parent;
        }
        path
    }

    fn walk (&self, path: &[String]) -> Option<Box<&Store>> {
        if path.len() == 0 { return Some(Box::new(self as &Store)) }
        let child_name = path.get(0).unwrap();
        let child_path = path.get(1..path.len()).unwrap();
        if let Some(child_store) = self.children.iter().find(|&&store| store.name == child_name) {
            Some(Box::new(child_store))
        } else {
            None
        }
    }
}

impl Store for Tree {
    fn get (&self, path: &[String]) -> Result<Value, StoreError> {
        Ok(Value::Null)
    }

    fn set (&self, path: &[String], value: Value) -> Result<(), StoreError> {
        Ok(())
    }
}

struct Json {
    name: String,
    parent: Tree
}

impl Json {
    fn path (&self) -> Vec<String> {
        let path = self.parent.path();
        path.push(self.name);
        path
    }
}

impl Store for Json {
    fn get (&self, path: &[String]) -> Result<Value, StoreError> {
        Ok(Value::Null)
    }

    fn set (&self, path: &[String], value: Value) -> Result<(), StoreError> {
        Ok(())
    }
}

fn main() {
    let foo = Json {
        name: "foo".into()
    };
    let root = Tree {
        name: "test".into(),
        children: vec![
            foo
        ]
    };
}

struct Database {
    store: DatabaseStore
}

impl Database {
    pub fn new (store: DatabaseStore) -> Self {
        Database {
            store
        }
    }

    pub fn get (&self, path: Vec<String>) -> Result<Option<Value>, StoreError> {
        let traversed = traverse_tree(&path, &self.store);
        if traversed.is_none() { return Ok(None) }
        let (extra_path, store) = traversed.unwrap();

        let depth = path.len() - extra_path.len();
        let value = get_from_store(store, &path, depth)?;

        Ok(Some(value))
    }
}

fn read_file (path: String) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    Ok(data)
}

fn traverse_tree <'a, 'b> (path: &'a [String], store: &'b DatabaseStore) -> Option<(&'a [String], &'b DatabaseStore)> {
    match store {
        DatabaseStore::Tree(map) => {
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

fn get_from_store (store: &DatabaseStore, path: &[String], depth: usize) -> Result<Value, StoreError> {
    if let DatabaseStore::Tree(map) = store {
        let mut next_map = BTreeMap::new();
        map.into_iter().try_for_each(|(key, store)| -> Result<(), StoreError> {
            let value = get_from_store(store, path, depth + 1)?;
            next_map.insert(key.clone(), value);
            Ok(())
        })?;
        return Ok(Value::Object(next_map));
    }

    let file_path = path.get(0..depth).unwrap();
    let file_extension = store_file_extension(store)?;
    let file_path: String = String::from("./") + &file_path.join("/") + &file_extension;
    let data = read_file(file_path)?;
    let value = store_value(store, &data)?;

    let value_path = path.get(depth..path.len()).unwrap();
    get_in_value(value_path, value)
}

fn get_in_value (path: &[String], value: Value) -> Result<Value, StoreError> {
    if path.len() == 0 { return Ok(value); }
    match value {
        Value::Object(object) => {
            let key = path.get(0).unwrap();
            let next_path = path.get(1..path.len()).unwrap();
            let next_value = object.get(key).ok_or(StoreError::NotFound)?;
            get_in_value(next_path, next_value.clone())
        },
        _ => Ok(value)
    }
}

fn store_file_extension (store: &DatabaseStore) -> Result<String, StoreError> {
    match store {
        DatabaseStore::Json => Ok(".json".into()),
        _ => return Err(StoreError::Unexpected)
    }
}

fn store_value (store: &DatabaseStore, data: &String) -> Result<Value, StoreError> {
    match store {
        DatabaseStore::Json => {
            let json_value: json::Value = json::from_str(&data)?;
            Ok(json_value.into())
        },
        _ => return Err(StoreError::Unexpected)
    }
}

#[derive(Debug)]
enum DatabaseStore {
    Tree(BTreeMap<String, DatabaseStore>),
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


#[derive(Debug)]
enum StoreError {
    Io(io::Error),
    Json(json::error::Error),
    NotFound,
    Unexpected
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StoreError::Io(ref err) => err.fmt(f),
            StoreError::Json(ref err) => err.fmt(f),
            StoreError::NotFound => write!(f, "Path not found"),
            StoreError::Unexpected => write!(f, "Unexpected (programmer) error"),
        }
    }
}

impl Error for StoreError {
    fn description(&self) -> &str {
        match *self {
            StoreError::Io(ref err) => err.description(),
            StoreError::Json(ref err) => err.description(),
            StoreError::NotFound => "not found",
            StoreError::Unexpected => "unexpected",
        }
    }
}

impl From<io::Error> for StoreError {
    fn from(err: io::Error) -> StoreError {
        StoreError::Io(err)
    }
}

impl From<json::error::Error> for StoreError {
    fn from(err: json::error::Error) -> StoreError {
        StoreError::Json(err)
    }
}
