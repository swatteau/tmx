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
use std::str::FromStr;

use xml::attribute::OwnedAttribute;

use error::Error;
use model::color::Color;
use model::data::Data;
use model::reader::{self, TmxReader, ElementReader};

#[derive(Debug, Default)]
pub struct Image {
    format: String,
    source: String,
    trans: Option<Color>,
    width: u32,
    height: u32,
    data: Option<Data>,
}

impl Image {
    pub fn format(&self) -> &str {
        &self.format
    }

    fn set_format<S: Into<String>>(&mut self, format: S) {
        self.format = format.into();
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    fn set_source<S: Into<String>>(&mut self, source: S) {
        self.source = source.into();
    }

    pub fn trans(&self) -> Option<&Color> {
        self.trans.as_ref()
    }

    fn set_trans(&mut self, color: Color) {
        self.trans = Some(color);
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    pub fn data(&self) -> Option<&Data> {
        self.data.as_ref()
    }

    fn set_data(&mut self, data: Data) {
        self.data = Some(data);
    }
}

impl<R: Read> ElementReader<Image> for TmxReader<R> {
    fn read_attributes(&mut self, image: &mut Image, name: &str, value: &str) -> ::Result<()> {
        match name {
            "format" => {
                image.set_format(value);
            }
            "source" => {
                image.set_source(value);
            }
            "trans" => {
                let color = Color::from_str(value)?;
                image.set_trans(color);
            }
            "width" => {
                let width = reader::read_num(value)?;
                image.set_width(width);
            }
            "height" => {
                let height = reader::read_num(value)?;
                image.set_height(height);
            }
            _ => {
                return Err(Error::UnknownAttribute(name.to_string()));
            }
        };
        Ok(())
    }

    fn read_children(&mut self, image: &mut Image, name: &str, attributes: &[OwnedAttribute]) -> ::Result<()>{
        if let "data" = name {
            let data = self.on_data(attributes)?;
            image.set_data(data);
        }
        Ok(())
    }
}

