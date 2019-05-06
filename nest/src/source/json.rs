use std::iter::FromIterator;

use indexmap::IndexMap;
use serde_json as json;

use super::FileSource;
use crate::Value;

#[derive(Clone, Debug)]
pub struct Json {}

impl FileSource for Json {
    type Value = json::Value;
    type SerError = json::Error;
    type DeError = json::Error;

    fn extension(&self) -> String {
        "json".into()
    }

    fn deserialize(&self, string: &str) -> Result<Self::Value, Self::DeError> {
        json::from_str(&string)
    }

    fn serialize(&self, value: &Self::Value) -> Result<String, Self::SerError> {
        let mut string = json::to_string_pretty(value)?;
        string.push('\n');
        Ok(string)
    }
}

impl From<json::Value> for Value {
    fn from(value: json::Value) -> Value {
        match value {
            json::Value::Null => Value::Null,
            json::Value::Bool(bool) => Value::Bool(bool),
            json::Value::Number(number) => {
                if number.is_u64() {
                    Value::Uint(number.as_u64().unwrap())
                } else if number.is_i64() {
                    Value::Int(number.as_i64().unwrap())
                } else {
                    Value::Float(number.as_f64().unwrap())
                }
            }
            json::Value::String(string) => Value::String(string),
            json::Value::Array(array) => {
                Value::Array(Vec::from_iter(array.into_iter().map(Self::from)))
            }
            json::Value::Object(object) => Value::Object(IndexMap::from_iter(
                object
                    .into_iter()
                    .map(|(key, value)| (key, Self::from(value))),
            )),
        }
    }
}

impl From<Value> for json::Value {
    fn from(value: Value) -> json::Value {
        match value {
            Value::Null => json::Value::Null,
            Value::Bool(bool) => json::Value::Bool(bool),
            Value::Int(int) => json::Value::Number(int.into()),
            Value::Uint(uint) => json::Value::Number(uint.into()),
            // will panic if float is NaN or Infinity
            // TODO, figure out how to handle this. maybe implement From<Option<Value>> for json::Value ?
            Value::Float(float) => match json::Number::from_f64(float) {
                Some(number) => json::Value::Number(number),
                None => panic!(".json source must not have NaN or Infinity numbers"),
            },
            Value::String(string) => json::Value::String(string),
            Value::Array(array) => {
                json::Value::Array(Vec::from_iter(array.into_iter().map(Self::from)))
            }
            Value::Object(object) => json::Value::Object(json::map::Map::from_iter(
                object
                    .into_iter()
                    .map(|(key, value)| (key, Self::from(value))),
            )),
        }
    }
}
