use std::convert::TryFrom;
use std::iter::FromIterator;
use std::str::FromStr;

use indexmap::IndexMap;
use toml;

use super::FileSource;
use crate::Value;
use snafu::Snafu;

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

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum IntoTomlError {
    #[snafu(display("Toml does not support null values"))]
    Null {},
}

impl TryFrom<Value> for toml::value::Value {
    type Error = IntoTomlError;

    fn try_from(value: Value) -> Result<toml::value::Value, IntoTomlError> {
        match value {
            Value::Null => Null {}.fail(),
            Value::Bool(bool) => Ok(toml::value::Value::Boolean(bool)),
            Value::Int(int) => Ok(toml::value::Value::Integer(int)),
            Value::Uint(uint) => Ok(toml::value::Value::Integer(uint as i64)),
            Value::Float(float) => Ok(toml::value::Value::Float(float)),
            Value::String(string) => Ok(match toml::value::Datetime::from_str(&string) {
                Ok(datetime) => toml::value::Value::Datetime(datetime),
                Err(_) => toml::value::Value::String(string),
            }),
            Value::Array(array) => {
                let mut next_array = Vec::with_capacity(array.len());
                array
                    .into_iter()
                    .try_for_each(|item| -> Result<(), IntoTomlError> {
                        let next_item = <Self as TryFrom<Value>>::try_from(item)?;
                        next_array.push(next_item);
                        Ok(())
                    })?;
                Ok(toml::value::Value::Array(next_array))
            }
            Value::Object(object) => {
                let mut next_map = toml::value::Table::with_capacity(object.len());
                object
                    .into_iter()
                    .try_for_each(|(key, value)| -> Result<(), IntoTomlError> {
                        let next_value = <Self as TryFrom<Value>>::try_from(value)?;
                        next_map.insert(key, next_value);
                        Ok(())
                    })?;
                Ok(toml::value::Value::Table(next_map))
            }
        }
    }
}
