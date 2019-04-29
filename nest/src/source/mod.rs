use std::fs::read_to_string;
use std::io::{self, Write};
use std::path::Path;

use atomicwrites::{AtomicFile, OverwriteBehavior};
use objekt;

use crate::error::Error;
use crate::value::Value;

mod hjson;
mod json;
mod yaml;

pub use hjson::Hjson;
pub use json::Json;
pub use yaml::Yaml;

pub trait Source: objekt::Clone + std::fmt::Debug {
    fn read(&self, path: &Path) -> Result<Value, Error>;
    fn write(&self, path: &Path, value: &Value) -> Result<(), Error>;
}

objekt::clone_trait_object!(Source);

pub trait FileSource: objekt::Clone + std::fmt::Debug {
    fn extension(&self) -> String;
    fn deserialize(&self, string: &str) -> Result<Value, Error>;
    fn serialize(&self, value: &Value) -> Result<String, Error>;
}

objekt::clone_trait_object!(FileSource);

impl<A: 'static> Source for A
where
    A: FileSource + Clone + std::fmt::Debug,
{
    fn read(&self, path: &Path) -> Result<Value, Error> {
        let file_path = path.with_extension(self.extension());
        let file_string = read_file(&file_path)?;
        self.deserialize(&file_string)
    }

    fn write(&self, path: &Path, value: &Value) -> Result<(), Error> {
        let file_path = path.with_extension(self.extension());
        let file_string = self.serialize(&value)?;
        write_file(&file_path, file_string)?;
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
