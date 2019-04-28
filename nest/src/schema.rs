use std::collections::BTreeMap;
use std::convert::From;
use std::iter::FromIterator;

use serde_hjson as hjson;
use serde_json as json;

use crate::error::Error;
use crate::value::Value;

/// The mapping of your data structures with the filesystem (files and directories).
///
/// A `Schema` is a tree with `Schema::Directory` as branches and files (e.g. `Schema::Json`) as leaves.
///
/// `Schema` also implements `From<serde_json::Value>`, so you can use the [`serde_json::json`] macro
/// to conveniently create schemas.
///
/// [`serde_json::json`](https://docs.serde.rs/serde_json/macro.json.html)
///

// NOTE schema should be a trait?

#[derive(Clone, Debug)]
pub enum Schema {
    Directory(BTreeMap<String, Schema>),
    Json,
    Hjson,
}

impl Schema {
    pub fn file_extension(&self) -> Result<String, Error> {
        match self {
            Schema::Json => Ok("json".into()),
            Schema::Hjson => Ok("hjson".into()),
            Schema::Directory(_) => Err(Error::Unexpected),
        }
    }

    // TODO better interface
    pub fn string_to_value(&self, data: &str) -> Result<Value, Error> {
        match self {
            Schema::Json => {
                let json_value: json::Value = json::from_str(&data)?;
                Ok(json_value.into())
            }
            Schema::Hjson => {
                let hjson_value: hjson::Value = hjson::from_str(&data)?;
                Ok(hjson_value.into())
            }
            Schema::Directory(_) => Err(Error::Unexpected),
        }
    }

    // TODO better interface
    pub fn value_to_string(&self, value: &Value) -> Result<String, Error> {
        match self {
            Schema::Json => {
                let json_value: json::Value = value.clone().into();
                let mut json_string = json::to_string_pretty(&json_value)?;
                json_string.push('\n');
                Ok(json_string)
            }
            Schema::Hjson => {
                let hjson_value: hjson::Value = value.clone().into();
                let hjson_string = hjson::to_string(&hjson_value)?;
                Ok(hjson_string)
            }
            Schema::Directory(_) => Err(Error::Unexpected),
        }
    }
}

// TODO implement From for Result<Schema>
// - https://github.com/rust-lang/rfcs/blob/master/text/1023-rebalancing-coherence.md
// - https://github.com/rust-lang/rfcs/blob/master/text/2451-re-rebalancing-coherence.md
// - wait until [re-rebalancing-coherence] feature is stable:
//     https://github.com/mssun/state-of-rust/blob/master/stable_library_feature.txt

impl From<Value> for Schema {
    fn from(value: Value) -> Schema {
        match value {
            Value::Object(object) => Schema::Directory(BTreeMap::from_iter(
                object
                    .into_iter()
                    .map(|(key, value)| (key, Self::from(value))),
            )),
            Value::String(string) => match string.as_str() {
                "json" => Schema::Json,
                "hjson" => Schema::Hjson,
                _ => panic!("Invalid string in json Schema: {:?}", string),
            },
            _ => panic!("Invalid value in json Schema: {:?}", value),
        }
    }
}

impl From<json::Value> for Schema {
    fn from(value: json::Value) -> Schema {
        let value: Value = value.into();
        value.into()
    }
}
