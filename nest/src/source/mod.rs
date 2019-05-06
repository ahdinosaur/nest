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

pub trait Source: objekt::Clone + std::fmt::Debug {
    fn read(&self, path: PathBuf) -> Result<Value, Error>;
    fn write(&self, path: PathBuf, value: &Value) -> Result<(), Error>;
}

objekt::clone_trait_object!(Source);

pub trait FileSource: objekt::Clone + std::fmt::Debug {
    fn extension(&self) -> String;
    fn deserialize(&self, string: &str) -> Result<Value, error::BoxError>;
    fn serialize(&self, value: &Value) -> Result<String, error::BoxError>;
}

objekt::clone_trait_object!(FileSource);

impl<A> Source for A
where
    A: FileSource + Clone + std::fmt::Debug,
{
    fn read(&self, path: PathBuf) -> Result<Value, Error> {
        let file_path = path.with_extension(self.extension());
        let file_string =
            read_file(&file_path).context(error::ReadSource { path: path.clone() })?;
        let value = self.deserialize(&file_string).context(error::Deserialize {
            path: path.clone(),
            string: file_string.clone(),
        })?;
        Ok(value)
    }

    fn write(&self, path: PathBuf, value: &Value) -> Result<(), Error> {
        let file_path = path.with_extension(self.extension());
        let file_string = self.serialize(&value).context(error::Serialize {
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
