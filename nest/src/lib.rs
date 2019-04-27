#![doc(html_root_url = "https://docs.rs/nest/0.3.2")]

//! # nest
//!
//! `nest` is a library to use your filesystem as a nested data store.
//!
//! The [`Store`](struct.Store.html) docs are a good place to get started.

pub use self::error::Error;
pub use self::store::Store;
pub use self::schema::Schema;
pub use self::value::Value;

mod error;
mod store;
mod schema;
mod value;
