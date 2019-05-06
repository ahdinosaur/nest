use serde_hjson as hjson;

use super::FileSource;
use crate::error::BoxError;

#[derive(Clone, Debug)]
pub struct Hjson {}

impl FileSource for Hjson {
    type Value = hjson::Value;

    fn extension(&self) -> String {
        "hjson".into()
    }

    fn deserialize(&self, string: &str) -> Result<Self::Value, BoxError> {
        let value = hjson::from_str(&string)?;
        Ok(value)
    }

    fn serialize(&self, value: &Self::Value) -> Result<String, BoxError> {
        let string = hjson::to_string(&value)?;
        Ok(string)
    }
}
