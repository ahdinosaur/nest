use std::convert::{From};
use std::collections::BTreeMap;
use std::iter::FromIterator;

use serde_json as json;

use crate::error::Error;

#[derive(Debug, Clone)]
pub enum Schema {
    Directory(BTreeMap<String, Schema>),
    Json
}

// TODO implement From for Result<Schema>
// - https://github.com/rust-lang/rfcs/blob/master/text/1023-rebalancing-coherence.md
// - https://github.com/rust-lang/rfcs/blob/master/text/2451-re-rebalancing-coherence.md
// - wait until [re-rebalancing-coherence] feature is stable:
//     https://github.com/mssun/state-of-rust/blob/master/stable_library_feature.txt
impl From<json::Value> for Schema {
    fn from(value: json::Value) -> Schema {
        match value {
            json::Value::Object(object) => Schema::Directory(BTreeMap::from_iter(object.into_iter().map(|(key, value)| (key, Self::from(value))))),
            json::Value::String(string) => {
                match string.as_str() {
                    "json" => Schema::Json,
                    _ => panic!("Invalid string in json Schema: {}", string),
                }
            },
            _ => panic!("Invalid value in json Schema: {}", value),
        }
    }
}