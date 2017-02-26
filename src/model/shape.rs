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
            let y = try!(coords.pop().unwrap());
            let x = try!(coords.pop().unwrap());
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
                    polygon.add_point(try!(result));
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
                    polyline.add_point(try!(result));
                }
            }
            _ => {
                return Err(Error::UnknownAttribute(name.to_string()));
            }
        };
        Ok(())
    }
}

