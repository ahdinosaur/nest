use std::collections::BTreeMap;
use std::convert::{TryFrom, TryInto};

use serde_json as json;
use snafu::OptionExt;

use crate::error::{self, Error, Result};
use crate::source::{Source, SOURCES};
use crate::value::Value;

/// The mapping of your data structures with the filesystem (files and directories).
///
/// A `Schema` is a tree with `Schema::Directory` as branches and sources (e.g. `Schema::Source`) as leaves.
///
/// `Schema` also implements `From<serde_json::Value>`, so you can use the [`serde_json::json`] macro
/// to conveniently create schemas.
///
/// [`serde_json::json`](https://docs.serde.rs/serde_json/macro.json.html)
///

#[derive(Clone, Debug)]
pub enum Schema {
    Directory(BTreeMap<String, Schema>),
    Source(Box<dyn Source>),
}

impl TryFrom<Value> for Schema {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self> {
        match value {
            Value::Object(object) => {
                let mut map = BTreeMap::new();
                object
                    .into_iter()
                    .try_for_each(|(key, value)| -> Result<()> {
                        let schema = Self::try_from(value)?;
                        map.insert(key, schema);
                        Ok(())
                    })?;
                Ok(Schema::Directory(map))
            }
            Value::String(string) => {
                let source: Box<dyn Source> = SOURCES
                    .iter()
                    .find_map(|source| {
                        if string == source.id() {
                            Some(source.clone())
                        } else {
                            None
                        }
                    })
                    .context(error::InvalidSchema {
                        value: Value::String(string),
                    })?;
                Ok(Schema::Source(source))
            }
            _ => Err(Error::InvalidSchema {
                value: value.clone(),
            }),
        }
    }
}

impl TryFrom<json::Value> for Schema {
    type Error = Error;

    fn try_from(value: json::Value) -> Result<Self> {
        let value: Value = value.into();
        value.try_into()
    }
}
