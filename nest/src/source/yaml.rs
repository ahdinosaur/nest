use std::convert::TryFrom;
use std::iter::FromIterator;

use indexmap::IndexMap;
use serde_yaml as yaml;
use snafu::{ensure, Snafu};

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

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum FromYamlError {
    #[snafu(display("Unable to convert Yaml with non-string key: {:#?}", key))]
    NonStringKey { key: yaml::Value },
}

impl TryFrom<yaml::Value> for Value {
    type Error = FromYamlError;

    fn try_from(value: yaml::Value) -> Result<Value, Self::Error> {
        match value {
            yaml::Value::Null => Ok(Value::Null),
            yaml::Value::Bool(bool) => Ok(Value::Bool(bool)),
            yaml::Value::Number(number) => Ok(if number.is_u64() {
                Value::Uint(number.as_u64().unwrap())
            } else if number.is_i64() {
                Value::Int(number.as_i64().unwrap())
            } else {
                Value::Float(number.as_f64().unwrap())
            }),
            yaml::Value::String(string) => Ok(Value::String(string)),
            yaml::Value::Sequence(sequence) => {
                let mut next_array = Vec::with_capacity(sequence.len());
                sequence
                    .into_iter()
                    .try_for_each(|item| -> Result<(), FromYamlError> {
                        let next_item = Self::try_from(item)?;
                        next_array.push(next_item);
                        Ok(())
                    })?;
                Ok(Value::Array(next_array))
            }
            yaml::Value::Mapping(mapping) => {
                let mut next_map = IndexMap::with_capacity(mapping.len());
                mapping
                    .into_iter()
                    .try_for_each(|(key, value)| -> Result<(), FromYamlError> {
                        ensure!(key.is_string(), NonStringKey { key: key.clone() });
                        let key = key.as_str().unwrap().to_owned();
                        let next_value = Self::try_from(value)?;
                        next_map.insert(key, next_value);
                        Ok(())
                    })?;
                Ok(Value::Object(next_map))
            }
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
