use serde_yaml as yaml;

use super::FileSource;
use crate::error::BoxError;
use crate::value::Value;

#[derive(Clone, Debug)]
pub struct Yaml {}

impl FileSource for Yaml {
    fn extension(&self) -> String {
        "yaml".into()
    }

    fn deserialize(&self, string: &str) -> Result<Value, BoxError> {
        let yaml_value: yaml::Value = yaml::from_str(&string)?;
        Ok(yaml_value.into())
    }

    fn serialize(&self, value: &Value) -> Result<String, BoxError> {
        let yaml_value: yaml::Value = value.clone().into();
        let mut yaml_string = yaml::to_string(&yaml_value)?;
        yaml_string.push('\n');
        Ok(yaml_string)
    }
}
