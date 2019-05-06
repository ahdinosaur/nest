use std::iter::FromIterator;
use std::str::FromStr;

use indexmap::IndexMap;
use toml;

use super::FileSource;
use crate::Value;

#[derive(Clone, Debug)]
pub struct Toml {}

impl FileSource for Toml {
    type Value = toml::Value;
    type SerError = toml::ser::Error;
    type DeError = toml::de::Error;

    fn extension(&self) -> String {
        "toml".into()
    }

    fn deserialize(&self, string: &str) -> Result<Self::Value, Self::DeError> {
        toml::from_str(&string)
    }

    fn serialize(&self, value: &Self::Value) -> Result<String, Self::SerError> {
        toml::to_string_pretty(&value)
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
