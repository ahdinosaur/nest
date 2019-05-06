use serde_yaml as yaml;

use super::FileSource;
use crate::error::BoxError;

#[derive(Clone, Debug)]
pub struct Yaml {}

impl FileSource for Yaml {
    type Value = yaml::Value;

    fn extension(&self) -> String {
        "yaml".into()
    }

    fn deserialize(&self, string: &str) -> Result<Self::Value, BoxError> {
        let value = yaml::from_str(&string)?;
        Ok(value)
    }

    fn serialize(&self, value: &Self::Value) -> Result<String, BoxError> {
        let mut string = yaml::to_string(&value)?;
        string.push('\n');
        Ok(string)
    }
}
