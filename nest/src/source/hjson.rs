use std::convert::TryFrom;
use std::iter::FromIterator;
use std::num::FpCategory;

use indexmap::IndexMap;
use serde_hjson as hjson;
use snafu::{ensure, Snafu};

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

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum IntoHjsonError {
    #[snafu(display("Hjson does not support NaN (Not a Number) number values"))]
    Nan {},
    #[snafu(display("Hjson does not support Infinity number values"))]
    Infinite {},
}

impl TryFrom<Value> for hjson::Value {
    type Error = IntoHjsonError;

    fn try_from(value: Value) -> Result<hjson::Value, Self::Error> {
        match value {
            Value::Null => Ok(hjson::Value::Null),
            Value::Bool(bool) => Ok(hjson::Value::Bool(bool)),
            Value::Int(int) => Ok(hjson::Value::I64(int)),
            Value::Uint(uint) => Ok(hjson::Value::U64(uint)),
            Value::Float(float) => {
                ensure!(float.classify() != FpCategory::Nan, Nan {});
                ensure!(float.classify() != FpCategory::Infinite, Infinite {});

                Ok(hjson::Value::F64(float))
            }
            Value::String(string) => Ok(hjson::Value::String(string)),
            Value::Array(array) => {
                let mut next_array = Vec::with_capacity(array.len());
                array
                    .into_iter()
                    .try_for_each(|item| -> Result<(), IntoHjsonError> {
                        let next_item = Self::try_from(item)?;
                        next_array.push(next_item);
                        Ok(())
                    })?;
                Ok(hjson::Value::Array(next_array))
            }
            Value::Object(object) => {
                let mut next_map = hjson::Map::with_capacity(object.len());
                object
                    .into_iter()
                    .try_for_each(|(key, value)| -> Result<(), IntoHjsonError> {
                        let next_value = Self::try_from(value)?;
                        next_map.insert(key, next_value);
                        Ok(())
                    })?;
                Ok(hjson::Value::Object(next_map))
            }
        }
    }
}
