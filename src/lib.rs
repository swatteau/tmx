//! TMX reading crate.
//!
//! This is a crate for reading TMX files.
//!
//! ```
//! use std::str::FromStr;
//! use tmx::*;
//!
//! let xml = "<map></<map>";
//! let map = Map::from_str(&xml);
//! ```

extern crate xml;

#[cfg(test)]
#[macro_use] extern crate assert_matches;

mod color;
mod error;
mod model;

pub use error::Error;
pub use model::*;

type Result<T> = std::result::Result<T, ::error::Error>;

