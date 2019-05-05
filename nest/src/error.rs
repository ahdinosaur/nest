use std::error;
use std::fmt;
use std::io;
use std::path;

use serde_hjson as hjson;
use serde_json as json;
use serde_yaml as yaml;
use snafu::{ensure, Backtrace, ErrorCompat, ResultExt, Snafu};
use toml;

use crate::path::Path;
use crate::value::Value;

pub type BoxError = Box<dyn std::error::Error>;

/// A specialized [`Error`] type for this crate's operations.
///
/// [`Error`]:  https://doc.rust-lang.org/stable/std/error/trait.Error.html
///

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error<'a> {
    #[snafu(display("Could not serialize string at {}: {}", path.display(), source))]
    Serialize {
        path: path::PathBuf,
        source: BoxError,
    },
    #[snafu(display("Could not deserialize value at {}: {}", path.display(), source))]
    Deserialize {
        path: path::PathBuf,
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
    GetSchema { path: Path<'a> },
    #[snafu(display("Value not found at {}", path))]
    GetValue { path: Path<'a> },
    #[snafu(display("Expected object value for directory schema at {}", path))]
    SetObjectValueWhenDirectory { path: Path<'a> },
    #[snafu(display("Invalid schema from value: {:#?}", value))]
    InvalidSchema { value: Value },
    #[snafu(display("Unexpected (programmer) error"))]
    Unexpected,
}

pub type Result<'a, A, B = Error<'a>> = std::result::Result<A, B>;
