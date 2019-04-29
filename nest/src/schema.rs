use std::collections::BTreeMap;
use std::convert::From;
use std::iter::FromIterator;
use std::ops::Deref;

use serde_json as json;

use crate::error::Error;
use crate::source::{self, Source};
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

// TODO implement From for Result<Schema>
// - https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types
//
// - https://github.com/rust-lang/rfcs/blob/master/text/1023-rebalancing-coherence.md
// - https://github.com/rust-lang/rfcs/blob/master/text/2451-re-rebalancing-coherence.md
// - wait until [re-rebalancing-coherence] feature is stable:
//     https://github.com/mssun/state-of-rust/blob/master/stable_library_feature.txt

impl From<Value> for WrappedResult<Schema> {
    fn from(value: Value) -> WrappedResult<Schema> {
        WrappedResult(match value {
            Value::Object(object) => Ok(Schema::Directory(BTreeMap::from_iter(
                object
                    .into_iter()
                    .map(|(key, value)| (key, Self::from(value)?)),
            ))),
            Value::String(string) => match string.as_str() {
                "json" => Ok(Schema::Source(Box::new(source::Json {}))),
                "hjson" => Ok(Schema::Source(Box::new(source::Hjson {}))),
                _ => Err(Error::Unexpected),
                // _ => panic!("Invalid string in json Schema: {:?}", string),
            },
            _ => Err(Error::Unexpected),
            // _ => panic!("Invalid value in json Schema: {:?}", value),
        })
    }
}

impl From<json::Value> for WrappedResult<Schema> {
    fn from(value: json::Value) -> WrappedResult<Schema> {
        let value: Value = value.into();
        value.into()
    }
}

struct WrappedResult<T>(Result<T, Error>);

impl<T> Deref for WrappedResult<T> {
    type Target = Result<T, Error>;

    fn deref(&self) -> &Result<T, Error> {
        &self.0
    }
}
