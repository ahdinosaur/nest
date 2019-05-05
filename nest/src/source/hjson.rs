use serde_hjson as hjson;

use super::FileSource;
use crate::error::BoxError;
use crate::value::Value;

#[derive(Clone, Debug)]
pub struct Hjson {}

impl FileSource for Hjson {
    fn extension(&self) -> String {
        "hjson".into()
    }

    fn deserialize(&self, string: &str) -> Result<Value, BoxError> {
        let hjson_value: hjson::Value = hjson::from_str(&string)?;
        Ok(hjson_value.into())
    }

    fn serialize(&self, value: &Value) -> Result<String, BoxError> {
        let hjson_value: hjson::Value = value.clone().into();
        let hjson_string = hjson::to_string(&hjson_value)?;
        Ok(hjson_string)
    }
}
