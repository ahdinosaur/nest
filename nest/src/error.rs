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

/// A specialized [`Error`] type for this crate's operations.
///
/// [`Error`]:  https://doc.rust-lang.org/stable/std/error/trait.Error.html
///

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error<'a> {
    #[snafu(display("Could not serialize string at {} into {}: {}", path.display(), source))]
    Serialize {
        path: &'a path::Path,
        source: SerdeError,
    },
    #[snafu(display("Could not deserialize value at {} into {}: {}", path.display(), source))]
    Deserialize {
        path: &'a path::Path,
        source: SerdeError,
    },
    #[snafu(display("Could not read file at {}: {}", path.display(), source))]
    ReadSource {
        path: &'a path::Path,
        source: io::Error,
    },
    #[snafu(display("Could not write file at {}: {}", path.display(), source))]
    WriteSource {
        path: &'a path::Path,
        source: io::Error,
    },
    #[snafu(display("Could not make directory at {}: {}", path.display(), source))]
    WriteDirectory {
        path: &'a path::Path,
        source: io::Error,
    },
    #[snafu(display("Schema not found at {}", path))]
    GetSchema { path: &'a Path<'a> },
    #[snafu(display("Value not found at {}", path))]
    GetValue { path: &'a Path<'a> },
    #[snafu(display("Expected object value for directory schema at {}", path))]
    SetObjectValueWhenDirectory { path: &'a Path<'a> },
    #[snafu(display("Invalid schema from value: {:#?}", value))]
    InvalidSchema { value: Value },
    #[snafu(display("Unexpected (programmer) error"))]
    Unexpected,
}

pub type Result<'a, A, B = Error<'a>> = std::result::Result<A, B>;

#[derive(Debug)]
pub struct SerdeError(pub Box<dyn error::Error>);

impl fmt::Display for SerdeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

/*
impl<A> From<A> for SerdeError
where
    A: error::Error,
{
    fn from(err: A) -> SerdeError {
        SerdeError(Box::new(err))
    }
}
*/

impl error::Error for SerdeError {
    fn description(&self) -> &str {
        self.0.description()
    }
}

// TODO better errors
//
// inspiration:
// - https://github.com/shepmaster/snafu
// - https://github.com/Keats/tera/blob/v1/src/errors.rs
// - https://github.com/Keats/kickstart/blob/master/src/errors.rs#L35-L55
