use std::io::Read;
use std::str::FromStr;
use xml::reader::{EventReader, XmlEvent};
use xml::attribute::OwnedAttribute;

use error::Error;
use super::*;

macro_rules! implement_handler {
    ($handler: ident, $tag: expr, $elem_type: ty) => {
        pub fn $handler(&mut self, attributes: &[OwnedAttribute]) -> ::Result<$elem_type> {
            let mut elem = <$elem_type>::default();

            // Process attributes
            for attr in attributes {
                try!(<Self as ElementReader<$elem_type>>::read_attributes(self, &mut elem, &attr.name.local_name, &attr.value));
            }

            // Process children elements
            while let Ok(event) = self.reader.next() {
                match event {
                    XmlEvent::StartElement { ref name, ref attributes, .. } => {
                        try!(<Self as ElementReader<$elem_type>>::read_children(self, &mut elem, &name.local_name, attributes));
                    }
                    XmlEvent::EndElement { ref name, .. } => {
                        if name.local_name == $tag {
                            break;
                        }
                    }
                    XmlEvent::Characters(ref content) => {
                        try!(<Self as ElementReader<$elem_type>>::read_content(self, &mut elem, &content));
                    }
                    XmlEvent::EndDocument { .. } => {
                        break;
                    }
                    _ => {}
                }
            }

            Ok(elem)
        }
    }
}

pub fn read_num<T: FromStr>(s: &str) -> ::Result<T> {
    s.parse::<T>().map_err(|_| Error::InvalidNumber(s.to_string()))
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> ::Result<Map> {
        let mut tmx = TmxReader::new(s.as_bytes());
        tmx.read_map()
    }
}

impl FromStr for Axis {
    type Err = Error;

    fn from_str(s: &str) -> ::Result<Axis> {
        match s {
            "x" => Ok(Axis::X),
            "y" => Ok(Axis::Y),
            _ => Err(Error::BadAxis(s.to_string())),
        }
    }
}

impl FromStr for Index {
    type Err = Error;

    fn from_str(s: &str) -> ::Result<Index> {
        match s {
            "even" => Ok(Index::Even),
            "odd" => Ok(Index::Odd),
            _ => Err(Error::BadIndex(s.to_string())),
        }
    }
}

impl FromStr for Orientation {
    type Err = Error;

    fn from_str(s: &str) -> ::Result<Orientation> {
        match s {
            "orthogonal" => Ok(Orientation::Orthogonal),
            "isometric" => Ok(Orientation::Isometric),
            "staggered" => Ok(Orientation::Staggered),
            "hexagonal" => Ok(Orientation::Hexagonal),
            _ => Err(Error::BadOrientation(s.to_string())),
        }
    }
}

impl FromStr for RenderOrder {
    type Err = Error;

    fn from_str(s: &str) -> ::Result<RenderOrder> {
        match s {
            "right-down" => Ok(RenderOrder::RightDown),
            "right-up" => Ok(RenderOrder::RightUp),
            "left-down" => Ok(RenderOrder::LeftDown),
            "left-up" => Ok(RenderOrder::LeftUp),
            _ => Err(Error::BadRenderOrder(s.to_string())),
        }
    }
}

impl FromStr for DrawOrder {
    type Err = Error;

    fn from_str(s: &str) -> ::Result<DrawOrder> {
        match s {
            "topdown" => Ok(DrawOrder::TopDown),
            "index" => Ok(DrawOrder::Index),
            _ => Err(Error::BadDrawOrder(s.to_string())),
        }
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

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> ::Result<Point> {
        let mut coords: Vec<_> = s.split(',').map(read_num::<i32>).collect();
        if coords.len() == 2 {
            let y = try!(coords.pop().unwrap());
            let x = try!(coords.pop().unwrap());
            Ok(Point {x: x, y: y})
        } else {
            Err(Error::InvalidPoint(s.to_string()))
        }
    }
}

impl FromStr for Corners {
    type Err = Error;

    fn from_str(s: &str) -> ::Result<Corners> {
        let ids: Vec<u32> = try!(s.split(',').map(read_num).collect());
        if ids.len() == 4 {
            Ok(Corners(ids[0], ids[1], ids[2], ids[3]))
        } else {
            Err(Error::InvalidTerrain(s.to_string()))
        }
    }
}

pub struct TmxReader<R: Read> {
    reader: EventReader<R>,
}

impl<R: Read> TmxReader<R> {

    pub fn new(source: R) -> TmxReader<R> {
        TmxReader {
            reader: EventReader::new(source),
        }
    }

    pub fn read_map(&mut self) -> ::Result<Map> {
        let mut result = Err(Error::BadXml);
        while let Ok(event) = self.reader.next() {
            match event {
                XmlEvent::StartElement { ref name, ref attributes, .. } => {
                    if name.local_name == "map" {
                        result = self.on_map(attributes);
                    }
                }
                XmlEvent::EndDocument { .. } => {
                    break;
                }
                _ => {}
            }
        }
        result
    }

    pub fn read_tileset(&mut self) -> ::Result<Tileset> {
        while let Ok(event) = self.reader.next() {
            match event {
                XmlEvent::StartElement { ref name, ref attributes, .. } => {
                    if name.local_name == "tileset" {
                        return self.on_tileset(attributes);
                    }
                }
                XmlEvent::EndDocument { .. } => {
                    break;
                }
                _ => {}
            }
        }
        Err(Error::BadXml)
    }

    implement_handler!(on_map, "map", Map);
    implement_handler!(on_tileset, "tileset", Tileset);
    implement_handler!(on_layer, "layer", Layer);
    implement_handler!(on_image_layer, "imagelayer", ImageLayer);
    implement_handler!(on_object_group, "objectgroup", ObjectGroup);
    implement_handler!(on_object, "object", Object);
    implement_handler!(on_image, "image", Image);
    implement_handler!(on_tile_offset, "tileoffset", TileOffset);
    implement_handler!(on_properties, "properties", PropertyCollection);
    implement_handler!(on_data, "data", Data);
    implement_handler!(on_data_tile, "tile", DataTile);
    implement_handler!(on_terrain_types, "terraintypes", TerrainCollection);
    implement_handler!(on_tile, "tile", Tile);
    implement_handler!(on_property, "property", Property);
    implement_handler!(on_terrain, "terrain", Terrain);
    implement_handler!(on_animation, "animation", Animation);
    implement_handler!(on_frame, "frame", Frame);
    implement_handler!(on_polygon, "polygon", Polygon);
    implement_handler!(on_polyline, "polyline", Polyline);
}

pub trait ElementReader<T> {
    #[allow(unused_variables)]
    fn read_attributes(&mut self, elem: &mut T, name: &str, value: &str) -> ::Result<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn read_children(&mut self, elem: &mut T, name: &str, attributes: &[OwnedAttribute]) -> ::Result<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn read_content(&mut self, elem: &mut T, content: &str) -> ::Result<()> {
        Ok(())
    }
}

impl<R: Read> ElementReader<Map> for TmxReader<R> {
    fn read_attributes(&mut self, map: &mut Map, name: &str, value: &str) -> ::Result<()> {
        match name {
            "backgroundcolor" => {
                let color = try!(Color::from_str(value));
                map.set_background_color(color);
            }
            "version" => {
                map.set_version(value);
            }
            "orientation" => {
                let orientation = try!(Orientation::from_str(value));
                map.set_orientation(orientation);
            }
            "renderorder" => {
                let render_order = try!(RenderOrder::from_str(value));
                map.set_render_order(render_order);
            }
            "width" => {
                let width = try!(read_num(value));
                map.set_width(width);
            }
            "height" => {
                let height = try!(read_num(value));
                map.set_height(height);
            }
            "tilewidth" => {
                let tile_width = try!(read_num(value));
                map.set_tile_width(tile_width);
            }
            "tileheight" => {
                let tile_height = try!(read_num(value));
                map.set_tile_height(tile_height);
            }
            "hexsidelength" => {
                let hex_side_length = try!(read_num(value));
                map.set_hex_side_length(hex_side_length);
            }
            "staggeraxis" => {
                let stagger_axis = try!(Axis::from_str(value));
                map.set_stagger_axis(stagger_axis);
            }
            "staggerindex" => {
                let stagger_index = try!(Index::from_str(value));
                map.set_stagger_index(stagger_index);
            }
            "nextobjectid" => {
                let next_object_id = try!(read_num(value));
                map.set_next_object_id(next_object_id);
            }
            _ => {
                return Err(Error::UnknownAttribute(name.to_string()));
            }
        };
        Ok(())
    }

    fn read_children(&mut self, map: &mut Map, name: &str, attributes: &[OwnedAttribute]) -> ::Result<()>{
        match name {
            "tileset" => {
                let ts = try!(self.on_tileset(attributes));
                map.add_tileset(ts);
            }
            "layer" => {
                let layer = try!(self.on_layer(attributes));
                map.add_layer(layer);
            }
            "imagelayer" => {
                let image_layer = try!(self.on_image_layer(attributes));
                map.add_image_layer(image_layer);
            }
            "objectgroup" => {
                let object_group = try!(self.on_object_group(attributes));
                map.add_object_group(object_group);
            }
            _ => {}
        }
        Ok(())
    }
}

impl<R: Read> ElementReader<Layer> for TmxReader<R> {
    fn read_attributes(&mut self, layer: &mut Layer, name: &str, value: &str) -> ::Result<()> {
        match name {
            "name" => {
                layer.set_name(value);
            }
            "opacity" => {
                let opacity = try!(read_num(value));
                layer.set_opacity(opacity);
            }
            "visible" => {
                let visibility = try!(read_num::<u32>(value));
                if visibility == 0 {
                    layer.set_visible(false);
                }
            }
            "offsetx" => {
                let offset_x = try!(read_num(value));
                layer.set_offset_x(offset_x);
            }
            "offsety" => {
                let offset_y = try!(read_num(value));
                layer.set_offset_y(offset_y);
            }
            "x" => {
                let x = try!(read_num(value));
                layer.set_x(x);
            }
            "y" => {
                let y = try!(read_num(value));
                layer.set_y(y);
            }
            "width" => {
                let width = try!(read_num(value));
                layer.set_width(width);
            }
            "height" => {
                let height = try!(read_num(value));
                layer.set_height(height);
            }
            _ => {
                return Err(Error::UnknownAttribute(name.to_string()));
            }
        };
        Ok(())
    }

    fn read_children(&mut self, layer: &mut Layer, name: &str, attributes: &[OwnedAttribute]) -> ::Result<()>{
        match name {
            "properties" => {
                let properties = try!(self.on_properties(attributes));
                layer.set_properties(properties);
            }
            "data" => {
                let data = try!(self.on_data(attributes));
                layer.set_data(data);
            }
            _ => {}
        };
        Ok(())
    }
}

impl<R: Read> ElementReader<ImageLayer> for TmxReader<R> {
    fn read_attributes(&mut self, image_layer: &mut ImageLayer, name: &str, value: &str) -> ::Result<()> {
        match name {
            "name" => {
                image_layer.set_name(value);
            }
            "opacity" => {
                let opacity = try!(read_num(value));
                image_layer.set_opacity(opacity);
            }
            "visible" => {
                let visibility = try!(read_num::<u32>(value));
                if visibility == 0 {
                    image_layer.set_visible(false);
                }
            }
            "offsetx" => {
                let offset_x = try!(read_num(value));
                image_layer.set_offset_x(offset_x);
            }
            "offsety" => {
                let offset_y = try!(read_num(value));
                image_layer.set_offset_y(offset_y);
            }
            "x" => {
                let x = try!(read_num(value));
                image_layer.set_x(x);
            }
            "y" => {
                let y = try!(read_num(value));
                image_layer.set_y(y);
            }
            "width" => {
                let width = try!(read_num(value));
                image_layer.set_width(width);
            }
            "height" => {
                let height = try!(read_num(value));
                image_layer.set_height(height);
            }
            _ => {
                return Err(Error::UnknownAttribute(name.to_string()));
            }
        };
        Ok(())
    }

    fn read_children(&mut self, image_layer: &mut ImageLayer, name: &str, attributes: &[OwnedAttribute]) -> ::Result<()>{
        match name {
            "properties" => {
                let properties = try!(self.on_properties(attributes));
                image_layer.set_properties(properties);
            }
            "image" => {
                let image = try!(self.on_image(attributes));
                image_layer.set_image(image);
            }
            _ => {}
        };
        Ok(())
    }
}

impl<R: Read> ElementReader<ObjectGroup> for TmxReader<R> {
    fn read_attributes(&mut self, object_group: &mut ObjectGroup, name: &str, value: &str) -> ::Result<()> {
        match name {
            "name" => {
                object_group.set_name(value);
            }
            "color" => {
                let color = try!(Color::from_str(value));
                object_group.set_color(color);
            }
            "opacity" => {
                let opacity = try!(read_num(value));
                object_group.set_opacity(opacity);
            }
            "visible" => {
                let visibility = try!(read_num::<u32>(value));
                if visibility == 0 {
                    object_group.set_visible(false);
                }
            }
            "offsetx" => {
                let offset_x = try!(read_num(value));
                object_group.set_offset_x(offset_x);
            }
            "offsety" => {
                let offset_y = try!(read_num(value));
                object_group.set_offset_y(offset_y);
            }
            "draworder" => {
                let draw_order = try!(DrawOrder::from_str(value));
                object_group.set_draw_order(draw_order);
            }
            "x" => {
                let x = try!(read_num(value));
                object_group.set_x(x);
            }
            "y" => {
                let y = try!(read_num(value));
                object_group.set_y(y);
            }
            "width" => {
                let width = try!(read_num(value));
                object_group.set_width(width);
            }
            "height" => {
                let height = try!(read_num(value));
                object_group.set_height(height);
            }
            _ => {
                return Err(Error::UnknownAttribute(name.to_string()));
            }
        };
        Ok(())
    }

    fn read_children(&mut self, object_group: &mut ObjectGroup, name: &str, attributes: &[OwnedAttribute]) -> ::Result<()>{
        match name {
            "object" => {
                let object = try!(self.on_object(attributes));
                object_group.add_object(object);
            }
            "properties" => {
                let properties = try!(self.on_properties(attributes));
                object_group.set_properties(properties);
            }
            _ => {}
        };
        Ok(())
    }
}

impl<R: Read> ElementReader<Object> for TmxReader<R> {
    fn read_attributes(&mut self, object: &mut Object, name: &str, value: &str) -> ::Result<()> {
        match name {
            "id" => {
                let id = try!(read_num(value));
                object.set_id(id);
            }
            "name" => {
                object.set_name(value);
            }
            "type" => {
                object.set_object_type(value);
            }
            "x" => {
                let x = try!(read_num(value));
                object.set_x(x);
            }
            "y" => {
                let y = try!(read_num(value));
                object.set_y(y);
            }
            "width" => {
                let width = try!(read_num(value));
                object.set_width(width);
            }
            "height" => {
                let height = try!(read_num(value));
                object.set_height(height);
            }
            "rotation" => {
                let rotation = try!(read_num(value));
                object.set_rotation(rotation);
            }
            "visible" => {
                let visibility = try!(read_num::<u32>(value));
                if visibility == 0 {
                    object.set_visible(false);
                }
            }
            "gid" => {
                let gid = try!(read_num(value));
                object.set_gid(gid);
            }
            _ => {
                return Err(Error::UnknownAttribute(name.to_string()));
            }
        };
        Ok(())
    }

    fn read_children(&mut self, object: &mut Object, name: &str, attributes: &[OwnedAttribute]) -> ::Result<()>{
        match name {
            "properties" => {
                let properties = try!(self.on_properties(attributes));
                object.set_properties(properties);
            }
            "ellipse" => {
                object.set_shape(Shape::Ellipse);
            }
            "polygon" => {
                let polygon = try!(self.on_polygon(attributes));
                object.set_shape(polygon);
            }
            "polyline" => {
                let polyline = try!(self.on_polyline(attributes));
                object.set_shape(polyline);
            }
            _ => {}
        };
        Ok(())
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
