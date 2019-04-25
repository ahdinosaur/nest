use std::collections::BTreeMap;
use std::convert::{From};
use std::iter::FromIterator;

use serde_json as json;
use serde_json::Number;

#[derive(Debug, Clone)]
pub enum Value {
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
