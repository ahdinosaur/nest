use serde_hjson as hjson;

use super::FileSource;

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
