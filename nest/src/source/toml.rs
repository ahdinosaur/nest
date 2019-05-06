use toml;

use super::FileSource;
use crate::error::BoxError;

#[derive(Clone, Debug)]
pub struct Toml {}

impl FileSource for Toml {
    type Value = toml::Value;

    fn extension(&self) -> String {
        "toml".into()
    }

    fn deserialize(&self, string: &str) -> Result<Self::Value, BoxError> {
        let value = toml::from_str(&string)?;
        Ok(value)
    }

    fn serialize(&self, value: &Self::Value) -> Result<String, BoxError> {
        let string = toml::to_string_pretty(&value)?;
        Ok(string)
    }
}
