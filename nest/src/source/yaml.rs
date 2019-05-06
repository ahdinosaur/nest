use std::iter::FromIterator;

use indexmap::IndexMap;
use serde_yaml as yaml;

use super::FileSource;
use crate::Value;

#[derive(Clone, Debug)]
pub struct Yaml {}

impl FileSource for Yaml {
    type Value = yaml::Value;
    type SerError = yaml::Error;
    type DeError = yaml::Error;

    fn extension(&self) -> String {
        "yaml".into()
    }

    fn deserialize(&self, string: &str) -> Result<Self::Value, Self::DeError> {
        yaml::from_str(&string)
    }

    fn serialize(&self, value: &Self::Value) -> Result<String, Self::SerError> {
        let mut string = yaml::to_string(&value)?;
        string.push('\n');
        Ok(string)
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
