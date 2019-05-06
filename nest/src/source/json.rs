use std::convert::TryFrom;
use std::iter::FromIterator;
use std::num::FpCategory;

use indexmap::IndexMap;
use serde_json as json;
use snafu::{ensure, Snafu};

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

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum IntoJsonError {
    #[snafu(display("Json does not support NaN (Not a Number) number values"))]
    Nan {},
    #[snafu(display("Json does not support Infinity number values"))]
    Infinite {},
}

impl TryFrom<Value> for json::Value {
    type Error = IntoJsonError;

    fn try_from(value: Value) -> Result<json::Value, Self::Error> {
        match value {
            Value::Null => Ok(json::Value::Null),
            Value::Bool(bool) => Ok(json::Value::Bool(bool)),
            Value::Int(int) => Ok(json::Value::Number(int.into())),
            Value::Uint(uint) => Ok(json::Value::Number(uint.into())),
            Value::Float(float) => {
                ensure!(float.classify() != FpCategory::Nan, Nan {});
                ensure!(float.classify() != FpCategory::Infinite, Infinite {});

                Ok(json::Value::Number(json::Number::from_f64(float).unwrap()))
            }
            Value::String(string) => Ok(json::Value::String(string)),
            Value::Array(array) => {
                let mut next_array = Vec::with_capacity(array.len());
                array
                    .into_iter()
                    .try_for_each(|item| -> Result<(), IntoJsonError> {
                        let next_item = Self::try_from(item)?;
                        next_array.push(next_item);
                        Ok(())
                    })?;
                Ok(json::Value::Array(next_array))
            }
            Value::Object(object) => {
                let mut next_map = json::map::Map::with_capacity(object.len());
                object
                    .into_iter()
                    .try_for_each(|(key, value)| -> Result<(), IntoJsonError> {
                        let next_value = Self::try_from(value)?;
                        next_map.insert(key, next_value);
                        Ok(())
                    })?;
                Ok(json::Value::Object(next_map))
            }
        }
    }
}
