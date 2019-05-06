use std::error;
use std::io;
use std::path;

use snafu::Snafu;

use crate::path::Path;
use crate::value::Value;

pub type BoxError = Box<dyn error::Error>;

/// A specialized [`Error`] type for this crate's operations.
///
/// [`Error`]:  https://doc.rust-lang.org/stable/std/error/trait.Error.html
///

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("Could not serialize value at {}\n{:#?}\n{}", path.display(), value, source))]
    Serialize {
        path: path::PathBuf,
        value: Value,
        source: BoxError,
    },
    #[snafu(display("Could not deserialize string at {}\n{}\n{}", path.display(), string, source))]
    Deserialize {
        path: path::PathBuf,
        string: String,
        source: BoxError,
    },
    #[snafu(display("Could not read file at {}: {}", path.display(), source))]
    ReadSource {
        path: path::PathBuf,
        source: io::Error,
    },
    #[snafu(display("Could not write file at {}: {}", path.display(), source))]
    WriteSource {
        path: path::PathBuf,
        source: io::Error,
    },
    #[snafu(display("Could not make directory at {}: {}", path.display(), source))]
    MakeDirectory {
        path: path::PathBuf,
        source: io::Error,
    },
    #[snafu(display("Schema not found at {}", path))]
    GetSchema { path: Path },
    #[snafu(display("Value not found at {}", path))]
    GetValue { path: Path },
    #[snafu(display("Expected object value for directory schema at {}", path))]
    SetObjectValueWhenDirectory { path: Path },
    #[snafu(display("Invalid schema from value: {:#?}", value))]
    InvalidSchema { value: Value },
    #[snafu(display("Unexpected (programmer) error"))]
    Unexpected,
}

pub type Result<A, B = Error> = std::result::Result<A, B>;
