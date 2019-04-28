use std::collections::BTreeMap;
use std::convert::From;
use std::fs::read_to_string;
use std::io;
use std::io::Write;
use std::iter::FromIterator;
use std::path::Path;

use atomicwrites::{AtomicFile, OverwriteBehavior};
use serde_hjson as hjson;
use serde_json as json;

use crate::error::Error;
use crate::value::Value;

/// The mapping of your data structures with the filesystem (files and directories).
///
/// A `Schema` is a tree with `Schema::Directory` as branches and files (e.g. `Schema::Json`) as leaves.
///
/// `Schema` also implements `From<serde_json::Value>`, so you can use the [`serde_json::json`] macro
/// to conveniently create schemas.
///
/// [`serde_json::json`](https://docs.serde.rs/serde_json/macro.json.html)
///

// TODO add another layer of encapsulation for "sources"
// so you could have a source that isn't a file, i.e. a structured directory of files
//

#[derive(Clone, Debug)]
pub enum Schema {
    Directory(BTreeMap<String, Schema>),
    Source(Box<dyn SourceFormat>),
}

pub trait SourceFormat: SourceFormatClone + std::fmt::Debug {
    fn read(&self, path: &Path) -> Result<Value, Error>;
    fn write(&self, path: &Path, value: &Value) -> Result<(), Error>;
}

pub trait FileFormat: FileFormatClone + std::fmt::Debug {
    fn extension(&self) -> String;
    fn deserialize(&self, string: &str) -> Result<Value, Error>;
    fn serialize(&self, value: &Value) -> Result<String, Error>;
}

impl<A: 'static> SourceFormat for A
where
    A: FileFormat + Clone + std::fmt::Debug,
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

#[derive(Clone, Debug)]
struct Json {}
impl FileFormat for Json {
    fn extension(&self) -> String {
        "json".into()
    }

    fn deserialize(&self, string: &str) -> Result<Value, Error> {
        let json_value: json::Value = json::from_str(&string)?;
        Ok(json_value.into())
    }

    fn serialize(&self, value: &Value) -> Result<String, Error> {
        let json_value: json::Value = value.clone().into();
        let mut json_string = json::to_string_pretty(&json_value)?;
        json_string.push('\n');
        Ok(json_string)
    }
}

#[derive(Clone, Debug)]
struct Hjson {}
impl FileFormat for Hjson {
    fn extension(&self) -> String {
        "hjson".into()
    }

    fn deserialize(&self, string: &str) -> Result<Value, Error> {
        let hjson_value: hjson::Value = hjson::from_str(&string)?;
        Ok(hjson_value.into())
    }

    fn serialize(&self, value: &Value) -> Result<String, Error> {
        let hjson_value: hjson::Value = value.clone().into();
        let hjson_string = hjson::to_string(&hjson_value)?;
        Ok(hjson_string)
    }
}

// TODO implement From for Result<Schema>
// - https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#using-the-newtype-pattern-to-implement-external-traits-on-external-types
//
// - https://github.com/rust-lang/rfcs/blob/master/text/1023-rebalancing-coherence.md
// - https://github.com/rust-lang/rfcs/blob/master/text/2451-re-rebalancing-coherence.md
// - wait until [re-rebalancing-coherence] feature is stable:
//     https://github.com/mssun/state-of-rust/blob/master/stable_library_feature.txt

impl From<Value> for Schema {
    fn from(value: Value) -> Schema {
        match value {
            Value::Object(object) => Schema::Directory(BTreeMap::from_iter(
                object
                    .into_iter()
                    .map(|(key, value)| (key, Self::from(value))),
            )),
            Value::String(string) => match string.as_str() {
                "json" => Schema::Source(Box::new(Json {})),
                "hjson" => Schema::Source(Box::new(Hjson {})),
                _ => panic!("Invalid string in json Schema: {:?}", string),
            },
            _ => panic!("Invalid value in json Schema: {:?}", value),
        }
    }
}

impl From<json::Value> for Schema {
    fn from(value: json::Value) -> Schema {
        let value: Value = value.into();
        value.into()
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

/* what... */
// https://stackoverflow.com/a/30353928

pub trait SourceFormatClone {
    fn clone_box(&self) -> Box<SourceFormat>;
}

impl<T> SourceFormatClone for T
where
    T: 'static + SourceFormat + Clone,
{
    fn clone_box(&self) -> Box<SourceFormat> {
        Box::new(self.clone())
    }
}

impl Clone for Box<SourceFormat> {
    fn clone(&self) -> Box<SourceFormat> {
        self.clone_box()
    }
}

pub trait FileFormatClone {
    fn clone_box(&self) -> Box<FileFormat>;
}

impl<T> FileFormatClone for T
where
    T: 'static + FileFormat + Clone,
{
    fn clone_box(&self) -> Box<FileFormat> {
        Box::new(self.clone())
    }
}

impl Clone for Box<FileFormat> {
    fn clone(&self) -> Box<FileFormat> {
        self.clone_box()
    }
}
