use std::collections::BTreeMap;
use std::convert::From;
use std::iter::FromIterator;

use serde_json as json;

// use crate::error::Error;
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

impl From<Value> for Schema {
    fn from(value: Value) -> Schema {
        match value {
            Value::Object(object) => Schema::Directory(BTreeMap::from_iter(
                object
                    .into_iter()
                    .map(|(key, value)| (key, Self::from(value))),
            )),
            Value::String(string) => match string.as_str() {
                "json" => Schema::Source(Box::new(source::Json {})),
                "hjson" => Schema::Source(Box::new(source::Hjson {})),
                "yaml" => Schema::Source(Box::new(source::Yaml {})),
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
