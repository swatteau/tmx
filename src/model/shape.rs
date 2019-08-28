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

use error::Error;
use model::reader::{self, TmxReader, ElementReader};

define_iterator_wrapper!(Points, Point);

#[derive(Debug, PartialEq, Eq)]
pub enum Shape {
    Ellipse,
    Polygon(Polygon),
    Polyline(Polyline),
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Polygon {
    points: Vec<Point>,
}

impl Polygon {
    pub fn points(&self) -> Points {
        Points(self.points.iter())
    }

    fn add_point(&mut self, point: Point) {
        self.points.push(point);
    }
}

impl From<Polygon> for Shape {
    fn from(polygon: Polygon) -> Shape {
        Shape::Polygon(polygon)
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Polyline {
    points: Vec<Point>,
}

impl Polyline {
    pub fn points(&self) -> Points {
        Points(self.points.iter())
    }

    fn add_point(&mut self, point: Point) {
        self.points.push(point);
    }
}

impl From<Polyline> for Shape {
    fn from(polyline: Polyline) -> Shape {
        Shape::Polyline(polyline)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> ::Result<Point> {
        let mut coords: Vec<_> = s.split(',').map(reader::read_num::<i32>).collect();
        if coords.len() == 2 {
            let y = coords.pop().unwrap()?;
            let x = coords.pop().unwrap()?;
            Ok(Point {x: x, y: y})
        } else {
            Err(Error::InvalidPoint(s.to_string()))
        }
    }
}

impl<R: Read> ElementReader<Polygon> for TmxReader<R> {
    fn read_attributes(&mut self, polygon: &mut Polygon, name: &str, value: &str) -> ::Result<()> {
        match name {
            "points" => {
                for result in value.split(' ').map(Point::from_str) {
                    polygon.add_point(result?);
                }
            }
            _ => {
                return Err(Error::UnknownAttribute(name.to_string()));
            }
        };
        Ok(())
    }
}

impl<R: Read> ElementReader<Polyline> for TmxReader<R> {
    fn read_attributes(&mut self, polyline: &mut Polyline, name: &str, value: &str) -> ::Result<()> {
        match name {
            "points" => {
                for result in value.split(' ').map(Point::from_str) {
                    polyline.add_point(result?);
                }
            }
            _ => {
                return Err(Error::UnknownAttribute(name.to_string()));
            }
        };
        Ok(())
    }
}

