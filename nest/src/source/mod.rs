use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::fs::read_to_string;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use atomicwrites::{AtomicFile, OverwriteBehavior};
use objekt;
use snafu::ResultExt;

use crate::error::{self, Error};
use crate::value::Value;

mod hjson;
mod json;
mod toml;
mod yaml;

pub use self::hjson::Hjson;
pub use self::json::Json;
pub use self::toml::Toml;
pub use self::yaml::Yaml;

lazy_static! {
    pub static ref SOURCES: Vec<Box<dyn Source>> = vec![
        Box::new(Hjson {}),
        Box::new(Json {}),
        Box::new(Toml {}),
        Box::new(Yaml {}),
    ];
}

pub trait Source: Send + Sync + objekt::Clone + fmt::Debug {
    fn id(&self) -> String;
    fn read(&self, path: PathBuf) -> Result<Value, Error>;
    fn write(&self, path: PathBuf, value: &Value) -> Result<(), Error>;
}

objekt::clone_trait_object!(Source);

pub trait FileSource: Send + Sync + objekt::Clone + fmt::Debug {
    type Value: 'static + TryFrom<Value> + TryInto<Value> + fmt::Debug + Clone;
    type SerError: 'static + std::error::Error;
    type DeError: 'static + std::error::Error;

    fn extension(&self) -> String;
    fn deserialize(&self, string: &str) -> Result<Self::Value, Self::DeError>;
    fn serialize(&self, value: &Self::Value) -> Result<String, Self::SerError>;
}

impl<A> Source for A
where
    A: FileSource + Send + Sync + Clone + fmt::Debug,
    <<A as FileSource>::Value as TryInto<Value>>::Error: std::error::Error,
    <<A as FileSource>::Value as TryFrom<Value>>::Error: std::error::Error,
{
    fn id(&self) -> String {
        self.extension()
    }

    fn read(&self, path: PathBuf) -> Result<Value, Error> {
        let file_path = path.with_extension(self.extension());
        let file_string =
            read_file(&file_path).context(error::ReadSource { path: path.clone() })?;
        let file_value = self
            .deserialize(&file_string)
            .map_err(|err| -> Box<dyn std::error::Error> { Box::new(err) })
            .context(error::Deserialize {
                kind: self.extension(),
                path: path.clone(),
                string: file_string.clone(),
            })?;
        let value: Value = file_value
            .clone()
            .try_into()
            .map_err(|err| -> Box<dyn std::error::Error> { Box::new(err) })
            .context(error::IntoValue {
                kind: self.extension(),
                path: path.clone(),
                value: Box::new(file_value.clone()) as Box<dyn fmt::Debug>,
            })?;
        Ok(value)
    }

    fn write(&self, path: PathBuf, value: &Value) -> Result<(), Error> {
        let file_path = path.with_extension(self.extension());
        let file_value = value
            .clone()
            .try_into()
            .map_err(|err| -> Box<dyn std::error::Error> { Box::new(err) })
            .context(error::FromValue {
                kind: self.extension(),
                path: path.clone(),
                value: value.clone(),
            })?;
        let file_string = self
            .serialize(&file_value)
            .map_err(|err| -> Box<dyn std::error::Error> { Box::new(err) })
            .context(error::Serialize {
                kind: self.extension(),
                path: path.clone(),
                value: value.clone(),
            })?;
        write_file(&file_path, file_string).context(error::WriteSource { path: path.clone() })?;
        Ok(())
    }
}

/* utils */
fn read_file(path: &Path) -> Result<String, io::Error> {
    read_to_string(path)
}

fn write_file(path: &Path, data: String) -> Result<(), io::Error> {
    let atomic_file = AtomicFile::new(path, OverwriteBehavior::AllowOverwrite);
    match atomic_file.write(|file| file.write_all(data.as_bytes())) {
        Ok(()) => Ok(()),
        Err(atomicwrites::Error::Internal(io_error)) => Err(io_error),
        Err(atomicwrites::Error::User(io_error)) => Err(io_error),
    }
}
