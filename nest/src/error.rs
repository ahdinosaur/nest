use std::io;
use std::fmt;

use serde_json as json;

/// A specialized [`Error`] type for this crate's operations.
///
/// [`Error`]:  https://doc.rust-lang.org/stable/std/error/trait.Error.html
#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Json(json::error::Error),
    NotFoundInSchema,
    NotFoundInValue,
    ExpectedObjectValueForDirectorySchema,
    Unexpected
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => err.fmt(f),
            Error::Json(ref err) => err.fmt(f),
            Error::NotFoundInSchema => write!(f, "Path not found in schema"),
            Error::NotFoundInValue => write!(f, "Path not found in value"),
            Error::ExpectedObjectValueForDirectorySchema => write!(f, "Expected object Value for Schema::Directory"),
            Error::Unexpected => write!(f, "Unexpected (programmer) error"),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref err) => err.description(),
            Error::Json(ref err) => err.description(),
            Error::NotFoundInSchema => "not found in schema",
            Error::NotFoundInValue => "not found in value",
            Error::ExpectedObjectValueForDirectorySchema => "expected object value for directory schema",
            Error::Unexpected => "programmer error",
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<json::error::Error> for Error {
    fn from(err: json::error::Error) -> Error {
        Error::Json(err)
    }
}

