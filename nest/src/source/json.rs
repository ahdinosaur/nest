use serde_json as json;

use super::FileSource;
use crate::error::BoxError;

#[derive(Clone, Debug)]
pub struct Json {}

impl FileSource for Json {
    type Value = json::Value;

    fn extension(&self) -> String {
        "json".into()
    }

    fn deserialize(&self, string: &str) -> Result<Self::Value, BoxError> {
        let value = json::from_str(&string)?;
        Ok(value)
    }

    fn serialize(&self, value: &Self::Value) -> Result<String, BoxError> {
        let mut string = json::to_string_pretty(value)?;
        string.push('\n');
        Ok(string)
    }
}
