use serde_yaml as yaml;

use super::FileSource;

#[derive(Clone, Debug)]
pub struct Yaml {}

impl FileSource for Yaml {
    type Value = yaml::Value;
    type SerError = yaml::Error;
    type DeError = yaml::Error;

    fn extension(&self) -> String {
        "yaml".into()
    }

    fn deserialize(&self, string: &str) -> Result<Self::Value, Self::DeError> {
        yaml::from_str(&string)
    }

    fn serialize(&self, value: &Self::Value) -> Result<String, Self::SerError> {
        let mut string = yaml::to_string(&value)?;
        string.push('\n');
        Ok(string)
    }
}
