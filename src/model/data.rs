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
// limitations under the License.

use std::io::Read;

use xml::attribute::OwnedAttribute;

use error::Error;
use model::reader::{self, TmxReader, ElementReader};

define_iterator_wrapper!(DataTiles, DataTile);

#[derive(Debug, Default)]
pub struct Data {
    encoding: Option<String>,
    compression: Option<String>,
    raw: Option<String>,
    tiles: Vec<DataTile>,
}

impl Data {
    pub fn encoding(&self) -> Option<&str> {
        self.encoding.as_ref().map(String::as_str)
    }

    fn set_encoding<S: Into<String>>(&mut self, encoding: S) {
        self.encoding = Some(encoding.into());
    }

    pub fn compression(&self) -> Option<&str> {
        self.compression.as_ref().map(String::as_str)
    }

    fn set_compression<S: Into<String>>(&mut self, compression: S) {
        self.compression = Some(compression.into());
    }

    pub fn raw_content(&self) -> Option<&str> {
        self.raw.as_ref().map(String::as_str)
    }

    fn set_raw_content<S: Into<String>>(&mut self, content: S) {
        self.raw = Some(content.into());
    }

    pub fn tiles(&self) -> DataTiles {
        DataTiles(self.tiles.iter())
    }

    fn add_tile(&mut self, tile: DataTile) {
        self.tiles.push(tile);
    }
}

#[derive(Debug, Default)]
pub struct DataTile {
    gid: i32,
}

impl DataTile {
    fn set_gid(&mut self, gid: i32) {
        self.gid = gid;
    }
}

impl<R: Read> ElementReader<Data> for TmxReader<R> {
    fn read_attributes(&mut self, data: &mut Data, name: &str, value: &str) -> ::Result<()> {
        match name {
            "encoding" => {
                data.set_encoding(value);
            }
            "compression" => {
                data.set_compression(value);
            }
            _ => {
                return Err(Error::UnknownAttribute(name.to_string()));
            }
        };
        Ok(())
    }

    fn read_children(&mut self, data: &mut Data, name: &str, attributes: &[OwnedAttribute]) -> ::Result<()>{
        if name == "tile" {
            let tile = try!(self.on_data_tile(attributes));
            data.add_tile(tile);
        }
        Ok(())
    }

    fn read_content(&mut self, data: &mut Data, content: &str) -> ::Result<()> {
        data.set_raw_content(content);
        Ok(())
    }
}

impl<R: Read> ElementReader<DataTile> for TmxReader<R> {
    fn read_attributes(&mut self, tile: &mut DataTile, name: &str, value: &str) -> ::Result<()> {
        match name {
            "gid" => {
                let gid = try!(reader::read_num(value));
                tile.set_gid(gid);
            }
            _ => {
                return Err(Error::UnknownAttribute(name.to_string()));
            }
        };
        Ok(())
    }
}

