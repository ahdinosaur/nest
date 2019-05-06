use std::iter::FromIterator;

use indexmap::IndexMap;
use serde_hjson as hjson;

use super::FileSource;
use crate::Value;

#[derive(Clone, Debug)]
pub struct Hjson {}

impl FileSource for Hjson {
    type Value = hjson::Value;
    type SerError = hjson::Error;
    type DeError = hjson::Error;

    fn extension(&self) -> String {
        "hjson".into()
    }

    fn deserialize(&self, string: &str) -> Result<Self::Value, Self::DeError> {
        hjson::from_str(&string)
    }

    fn serialize(&self, value: &Self::Value) -> Result<String, Self::SerError> {
        hjson::to_string(&value)
    }
}

impl From<hjson::Value> for Value {
    fn from(value: hjson::Value) -> Value {
        match value {
            hjson::Value::Null => Value::Null,
            hjson::Value::Bool(bool) => Value::Bool(bool),
            hjson::Value::I64(int) => Value::Int(int),
            hjson::Value::U64(uint) => Value::Uint(uint),
            hjson::Value::F64(float) => Value::Float(float),
            hjson::Value::String(string) => Value::String(string),
            hjson::Value::Array(array) => {
                Value::Array(Vec::from_iter(array.into_iter().map(Self::from)))
            }
            hjson::Value::Object(object) => Value::Object(IndexMap::from_iter(
                object
                    .into_iter()
                    .map(|(key, value)| (key, Self::from(value))),
            )),
        }
    }
}

impl From<Value> for hjson::Value {
    fn from(value: Value) -> hjson::Value {
        match value {
            Value::Null => hjson::Value::Null,
            Value::Bool(bool) => hjson::Value::Bool(bool),
            Value::Int(int) => hjson::Value::I64(int),
            Value::Uint(uint) => hjson::Value::U64(uint),
            Value::Float(float) => hjson::Value::F64(float),
            Value::String(string) => hjson::Value::String(string),
            Value::Array(array) => {
                hjson::Value::Array(Vec::from_iter(array.into_iter().map(Self::from)))
            }
            Value::Object(object) => hjson::Value::Object(hjson::Map::from_iter(
                object
                    .into_iter()
                    .map(|(key, value)| (key, Self::from(value))),
            )),
        }
    }
}
