use std::io;
use std::fmt;

use serde_json as json;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Json(json::error::Error),
    NotFound,
    BadInput,
    Unexpected
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => err.fmt(f),
            Error::Json(ref err) => err.fmt(f),
            Error::NotFound => write!(f, "Path not found"),
            Error::BadInput => write!(f, "Bad input"),
            Error::Unexpected => write!(f, "Unexpected (programmer) error"),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref err) => err.description(),
            Error::Json(ref err) => err.description(),
            Error::NotFound => "not found",
            Error::Unexpected => "unexpected",
            Error::BadInput => "bad input",
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

