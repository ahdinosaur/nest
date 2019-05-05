use std::convert::From;
use std::iter::FromIterator;
use std::str::FromStr;

use indexmap::IndexMap;
use serde_hjson as hjson;
use serde_json as json;
use serde_yaml as yaml;

/// Represents any valid Nest value.
///
/// Similar to [`serde_json::Value`].
///
/// `Value` also implements `From<serde_json::Value>`, so you can use the [`serde_json::json`] macro
/// to conveniently create Nest values.
///
/// [`serde_json::Value`](https://docs.serde.rs/serde_json/value/enum.Value.html)
/// [`serde_json::json`](https://docs.serde.rs/serde_json/macro.json.html)

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Null, // maybe no null?
    Bool(bool),
    Int(i64), // maybe only float as number?
    Uint(u64),
    Float(f64),
    String(String),
    Array(Vec<Value>),
    Object(IndexMap<String, Value>), // maybe use IndexedHashMap to preserve order?
}

impl Value {
    pub fn is_object(&self) -> bool {
        self.as_object().is_some()
    }

    pub fn as_object(&self) -> Option<&IndexMap<String, Value>> {
        match *self {
            Value::Object(ref map) => Some(map),
            _ => None,
        }
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

impl From<yaml::Value> for Value {
    fn from(value: yaml::Value) -> Value {
        match value {
            yaml::Value::Null => Value::Null,
            yaml::Value::Bool(bool) => Value::Bool(bool),
            yaml::Value::Number(number) => {
                if number.is_u64() {
                    Value::Uint(number.as_u64().unwrap())
                } else if number.is_i64() {
                    Value::Int(number.as_i64().unwrap())
                } else {
                    Value::Float(number.as_f64().unwrap())
                }
            }
            yaml::Value::String(string) => Value::String(string),
            yaml::Value::Sequence(sequence) => {
                Value::Array(Vec::from_iter(sequence.into_iter().map(Self::from)))
            }
            yaml::Value::Mapping(mapping) => Value::Object(IndexMap::from_iter(
                mapping.into_iter().map(|(key, value)| {
                    if let yaml::Value::String(key) = key {
                        (key, Self::from(value))
                    } else {
                        // TODO, figure out how to handle this.
                        panic!(".yaml source may only have string keys in object (mapping)")
                    }
                }),
            )),
        }
    }
}

impl From<Value> for yaml::Value {
    fn from(value: Value) -> yaml::Value {
        match value {
            Value::Null => yaml::Value::Null,
            Value::Bool(bool) => yaml::Value::Bool(bool),
            Value::Int(int) => yaml::Value::Number(int.into()),
            Value::Uint(uint) => yaml::Value::Number(uint.into()),
            Value::Float(float) => yaml::Value::Number(float.into()),
            Value::String(string) => yaml::Value::String(string),
            Value::Array(array) => {
                yaml::Value::Sequence(Vec::from_iter(array.into_iter().map(Self::from)))
            }
            Value::Object(object) => yaml::Value::Mapping(yaml::Mapping::from_iter(
                object
                    .into_iter()
                    .map(|(key, value)| (Self::from(key), Self::from(value))),
            )),
        }
    }
}

impl From<toml::value::Value> for Value {
    fn from(value: toml::value::Value) -> Value {
        match value {
            toml::value::Value::Boolean(bool) => Value::Bool(bool),
            toml::value::Value::Integer(int) => Value::Int(int),
            toml::value::Value::Float(float) => Value::Float(float),
            toml::value::Value::String(string) => Value::String(string),
            toml::value::Value::Datetime(datetime) => Value::String(datetime.to_string()),
            toml::value::Value::Array(array) => {
                Value::Array(Vec::from_iter(array.into_iter().map(Self::from)))
            }
            toml::value::Value::Table(object) => Value::Object(IndexMap::from_iter(
                object
                    .into_iter()
                    .map(|(key, value)| (key, Self::from(value))),
            )),
        }
    }
}

impl From<Value> for toml::value::Value {
    fn from(value: Value) -> toml::value::Value {
        match value {
            Value::Null => panic!(".toml may not have null value"), // TODO oh no.
            Value::Bool(bool) => toml::value::Value::Boolean(bool),
            Value::Int(int) => toml::value::Value::Integer(int),
            Value::Uint(uint) => toml::value::Value::Integer(uint as i64),
            Value::Float(float) => toml::value::Value::Float(float),
            Value::String(string) => match toml::value::Datetime::from_str(&string) {
                Ok(datetime) => toml::value::Value::Datetime(datetime),
                Err(_) => toml::value::Value::String(string),
            },
            Value::Array(array) => {
                toml::value::Value::Array(Vec::from_iter(array.into_iter().map(Self::from)))
            }
            Value::Object(object) => toml::value::Value::Table(toml::value::Table::from_iter(
                object
                    .into_iter()
                    .map(|(key, value)| (key, Self::from(value))),
            )),
        }
    }
}
