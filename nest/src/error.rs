use std::fmt;
use std::io;

use serde_hjson as hjson;
use serde_json as json;
use serde_yaml as yaml;
use toml;

/// A specialized [`Error`] type for this crate's operations.
///
/// [`Error`]:  https://doc.rust-lang.org/stable/std/error/trait.Error.html
#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Json(json::error::Error),
    Hjson(hjson::Error),
    TomlDe(toml::de::Error),
    TomlSer(toml::ser::Error),
    Yaml(yaml::Error),
    NotFoundInSchema,
    NotFoundInValue,
    ExpectedObjectValueForDirectorySchema,
    Unexpected,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => err.fmt(f),
            Error::Json(ref err) => err.fmt(f),
            Error::Hjson(ref err) => err.fmt(f),
            Error::TomlDe(ref err) => err.fmt(f),
            Error::TomlSer(ref err) => err.fmt(f),
            Error::Yaml(ref err) => err.fmt(f),
            Error::NotFoundInSchema => write!(f, "Path not found in schema"),
            Error::NotFoundInValue => write!(f, "Path not found in value"),
            Error::ExpectedObjectValueForDirectorySchema => {
                write!(f, "Expected object Value for Schema::Directory")
            }
            Error::Unexpected => write!(f, "Unexpected (programmer) error"),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref err) => err.description(),
            Error::Json(ref err) => err.description(),
            Error::Hjson(ref err) => err.description(),
            Error::TomlDe(ref err) => err.description(),
            Error::TomlSer(ref err) => err.description(),
            Error::Yaml(ref err) => err.description(),
            Error::NotFoundInSchema => "not found in schema",
            Error::NotFoundInValue => "not found in value",
            Error::ExpectedObjectValueForDirectorySchema => {
                "expected object value for directory schema"
            }
            Error::Unexpected => "programmer error",
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

// TODO is there a general way to handle source errors?

impl From<json::error::Error> for Error {
    fn from(err: json::error::Error) -> Error {
        Error::Json(err)
    }
}

impl From<hjson::error::Error> for Error {
    fn from(err: hjson::error::Error) -> Error {
        Error::Hjson(err)
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Error {
        Error::TomlDe(err)
    }
}

impl From<toml::ser::Error> for Error {
    fn from(err: toml::ser::Error) -> Error {
        Error::TomlSer(err)
    }
}

impl From<yaml::Error> for Error {
    fn from(err: yaml::Error) -> Error {
        Error::Yaml(err)
    }
}
