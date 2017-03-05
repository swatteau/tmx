// This file is part of tmx
// Copyright 2017 Sébastien Watteau
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.use std::error;

//! A simple crate for reading [Tiled](http://www.mapeditor.org/) files.
//!
//! # Getting Started
//!
//! Reading a `.tmx` (map) file:
//!
//! ```
//! extern crate tmx;
//!
//! fn main() {
//!     match tmx::Map::open("some_file.tmx") {
//!         Ok(map) => println!("Got a map!"),
//!         Err(e) => println!("Got an error: {}", e)
//!     };
//! }
//! ```
//!
//! Reading a `.tsx` (tileset) file:
//!
//! ```
//! extern crate tmx;
//!
//! fn main() {
//!     match tmx::Tileset::open("some_file.tsx") {
//!         Ok(tileset) => println!("Got a tileset!"),
//!         Err(e) => println!("Got an error: {}", e)
//!     };
//! }
//! ```
//!
//! Reading data directly from a string:
//!
//! ```
//! extern crate tmx;
//!
//! use std::str::FromStr;
//!
//! let empty_map = tmx::Map::from_str(r#"<map version="1.0"/>"#);
//! ```

extern crate xml;

#[cfg(test)]
#[macro_use] extern crate assert_matches;

mod error;
mod model;

pub use error::Error;
pub use model::*;

pub type Result<T> = std::result::Result<T, ::error::Error>;

