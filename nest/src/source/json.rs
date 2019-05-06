use serde_json as json;

use super::FileSource;

#[derive(Clone, Debug)]
pub struct Json {}

impl FileSource for Json {
    type Value = json::Value;
    type SerError = json::Error;
    type DeError = json::Error;

    fn extension(&self) -> String {
        "json".into()
    }

    fn deserialize(&self, string: &str) -> Result<Self::Value, Self::DeError> {
        json::from_str(&string)
    }

    fn serialize(&self, value: &Self::Value) -> Result<String, Self::SerError> {
        let mut string = json::to_string_pretty(value)?;
        string.push('\n');
        Ok(string)
    }
}
