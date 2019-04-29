use toml;

use super::FileSource;
use crate::error::Error;
use crate::value::Value;

#[derive(Clone, Debug)]
pub struct Toml {}

impl FileSource for Toml {
    fn extension(&self) -> String {
        "toml".into()
    }

    fn deserialize(&self, string: &str) -> Result<Value, Error> {
        let toml_value: toml::Value = toml::from_str(&string)?;
        Ok(toml_value.into())
    }

    fn serialize(&self, value: &Value) -> Result<String, Error> {
        let toml_value: toml::Value = value.clone().into();
        let toml_string = toml::to_string_pretty(&toml_value)?;
        Ok(toml_string)
    }
}
