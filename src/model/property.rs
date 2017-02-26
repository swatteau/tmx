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
    String,
    Int,
    Float,
    Bool,
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
            "string" => Ok(PropertyType::String),
            "int" => Ok(PropertyType::Int),
            "float" => Ok(PropertyType::Float),
            "bool" => Ok(PropertyType::Bool),
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
            "value" => {
                property.set_value(value);
            }
            "type" => {
                property.set_property_type(try!(PropertyType::from_str(value)));
            }
            _ => {
                return Err(Error::UnknownAttribute(name.to_string()));
            }
        };
        Ok(())
    }
}

