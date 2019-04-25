use std::convert::{From};
use std::collections::BTreeMap;
use std::iter::FromIterator;

use serde_json as json;

use crate::error::Error;

#[derive(Debug, Clone)]
pub enum Store {
    Tree(BTreeMap<String, Store>),
    Json
}

// TODO implement From for Result<Store>
// - https://github.com/rust-lang/rfcs/blob/master/text/1023-rebalancing-coherence.md
// - https://github.com/rust-lang/rfcs/blob/master/text/2451-re-rebalancing-coherence.md
// - wait until [re-rebalancing-coherence] feature is stable:
//     https://github.com/mssun/state-of-rust/blob/master/stable_library_feature.txt
impl From<json::Value> for Store {
    fn from(value: json::Value) -> Store {
        match value {
            json::Value::Object(object) => Store::Tree(BTreeMap::from_iter(object.into_iter().map(|(key, value)| (key, Self::from(value))))),
            json::Value::String(string) => {
                match string.as_str() {
                    "json" => Store::Json,
                    _ => panic!("Invalid string in json Store: {}", string),
                }
            },
            _ => panic!("Invalid value in json Store: {}", value),
        }
    }
}
