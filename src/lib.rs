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

mod error;
mod model;

pub use error::Error;
pub use model::*;

pub type Result<T> = std::result::Result<T, ::error::Error>;

