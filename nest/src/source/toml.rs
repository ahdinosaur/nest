use toml;

use super::FileSource;

#[derive(Clone, Debug)]
pub struct Toml {}

impl FileSource for Toml {
    type Value = toml::Value;
    type SerError = toml::ser::Error;
    type DeError = toml::de::Error;

    fn extension(&self) -> String {
        "toml".into()
    }

    fn deserialize(&self, string: &str) -> Result<Self::Value, Self::DeError> {
        toml::from_str(&string)
    }

    fn serialize(&self, value: &Self::Value) -> Result<String, Self::SerError> {
        toml::to_string_pretty(&value)
    }
}
