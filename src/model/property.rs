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

use std::io::Read;
use std::str::FromStr;

use error::Error;
use model::reader::{TmxReader, ElementReader};

define_iterator_wrapper!(Properties, Property);

#[derive(Debug, Default)]
pub struct Property {
    name: String,
    value: String,
    property_type: PropertyType,
}

impl Property {
    pub fn new<S>(name: S, value: S, property_type: PropertyType) -> Property
        where S: Into<String>
    {
        Property {
            name: name.into(),
            value: value.into(),
            property_type: property_type,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    fn set_name<S: Into<String>>(&mut self, name: S) {
        self.name = name.into();
    }

    pub fn property_type(&self) -> PropertyType {
        self.property_type
    }

    fn set_property_type(&mut self, property_type: PropertyType) {
        self.property_type = property_type;
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    fn set_value<S: Into<String>>(&mut self, value: S) {
        self.value = value.into();
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PropertyType {
    Bool,
    Color,
    File,
    Float,
    Int,
    String,
}

impl Default for PropertyType {
    fn default() -> PropertyType {
        PropertyType::String
    }
}

#[derive(Debug, Default)]
pub struct PropertyCollection(Vec<Property>);

impl PropertyCollection {
    pub fn new() -> PropertyCollection {
        PropertyCollection(Vec::new())
    }

    pub fn push(&mut self, property: Property) {
        self.0.push(property);
    }

    pub fn iter(&self) -> Properties {
        Properties(self.0.iter())
    }
}

impl FromStr for PropertyType {
    type Err = Error;

    fn from_str(s: &str) -> ::Result<PropertyType> {
        match s {
            "bool" => Ok(PropertyType::Bool),
            "color" => Ok(PropertyType::Color),
            "file" => Ok(PropertyType::File),
            "float" => Ok(PropertyType::Float),
            "int" => Ok(PropertyType::Int),
            "string" => Ok(PropertyType::String),
            _ => Err(Error::BadPropertyType(s.to_string())),
        }
    }
}

impl<R: Read> ElementReader<Property> for TmxReader<R> {
    fn read_attributes(&mut self, property: &mut Property, name: &str, value: &str) -> ::Result<()> {
        match name {
            "name" => {
                property.set_name(value);
            }
            "type" => {
                property.set_property_type(try!(PropertyType::from_str(value)));
            }
            "value" => {
                property.set_value(value);
            }
            _ => {
                return Err(Error::UnknownAttribute(name.to_string()));
            }
        };
        Ok(())
    }
}

