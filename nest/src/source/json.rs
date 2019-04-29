use serde_json as json;

use super::FileSource;
use crate::error::Error;
use crate::value::Value;

#[derive(Clone, Debug)]
pub struct Json {}

impl FileSource for Json {
    fn extension(&self) -> String {
        "json".into()
    }

    fn deserialize(&self, string: &str) -> Result<Value, Error> {
        let json_value: json::Value = json::from_str(&string)?;
        Ok(json_value.into())
    }

    fn serialize(&self, value: &Value) -> Result<String, Error> {
        let json_value: json::Value = value.clone().into();
        let mut json_string = json::to_string_pretty(&json_value)?;
        json_string.push('\n');
        Ok(json_string)
    }
}
