use std::io::Read;
use std::str::FromStr;
use xml::reader::{EventReader, XmlEvent};
use xml::attribute::OwnedAttribute;

use ::error::Error;
use super::*;

macro_rules! implement_handler {
    ($handler: ident, $tag: expr, $elem_type: ty) => {
        fn $handler(&mut self, attributes: &[OwnedAttribute]) -> ::Result<$elem_type> {
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

fn read_num<T: FromStr>(s: &str) -> ::Result<T> {
    s.parse::<T>().map_err(|_| Error::InvalidNumber(s.to_string()))
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> ::Result<Map> {
        let mut tmx = TmxReader::new(s.as_bytes());
        tmx.read_map()
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

impl FromStr for Tileset {
    type Err = Error;

    fn from_str(s: &str) -> ::Result<Tileset> {
        let mut tsx = TmxReader::new(s.as_bytes());
        tsx.read_tileset()
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
    implement_handler!(on_image, "image", Image);
    implement_handler!(on_tile_offset, "tileoffset", TileOffset);
    implement_handler!(on_properties, "properties", PropertySet);
    implement_handler!(on_terrain_types, "terraintypes", TerrainSet);
    implement_handler!(on_tile, "tile", Tile);
    implement_handler!(on_property, "property", Property);
    implement_handler!(on_terrain, "terrain", Terrain);
    implement_handler!(on_animation, "animation", Animation);
    implement_handler!(on_frame, "frame", Frame);
}

trait ElementReader<T> {
    #[allow(unused_variables)]
    fn read_attributes(&mut self, map: &mut T, name: &str, value: &str) -> ::Result<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn read_children(&mut self, map: &mut T, name: &str, attributes: &[OwnedAttribute]) -> ::Result<()> {
        Ok(())
    }
}

impl<R: Read> ElementReader<Map> for TmxReader<R> {
    fn read_attributes(&mut self, map: &mut Map, name: &str, value: &str) -> ::Result<()> {
        match name {
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

impl<R: Read> ElementReader<Tileset> for TmxReader<R> {
    fn read_attributes(&mut self, tileset: &mut Tileset, name: &str, value: &str) -> ::Result<()> {
        match name {
            "name" => {
                tileset.set_name(value);
            }
            "firstgid" => {
                let first_gid = try!(read_num(value));
                tileset.set_first_gid(first_gid);
            }
            "tilewidth" => {
                let tile_width = try!(read_num(value));
                tileset.set_tile_width(tile_width);
            }
            "tileheight" => {
                let tile_height = try!(read_num(value));
                tileset.set_tile_height(tile_height);
            }
            "spacing" => {
                let spacing = try!(read_num(value));
                tileset.set_spacing(spacing);
            }
            "margin" => {
                let margin = try!(read_num(value));
                tileset.set_margin(margin);
            }
            "tilecount" => {
                let tile_count = try!(read_num(value));
                tileset.set_tile_count(tile_count);
            }
            "columns" => {
                let columns = try!(read_num(value));
                tileset.set_columns(columns);
            }
            _ => {
                return Err(Error::UnknownAttribute(name.to_string()));
            }
        };
        Ok(())
    }

    fn read_children(&mut self, tileset: &mut Tileset, name: &str, attributes: &[OwnedAttribute]) -> ::Result<()>{
        match name {
            "image" => {
                let image = try!(self.on_image(attributes));
                tileset.set_image(image);
            }
            "tileoffset" => {
                let tile_offset = try!(self.on_tile_offset(attributes));
                tileset.set_tile_offset(tile_offset);
            }
            "properties" => {
                let properties = try!(self.on_properties(attributes));
                tileset.set_properties(properties);
            }
            "terraintypes" => {
                let terrain_types = try!(self.on_terrain_types(attributes));
                tileset.set_terrain_types(terrain_types);
            }
            "tile" => {
                let tile = try!(self.on_tile(attributes));
                tileset.add_tile(tile);
            }
            _ => {}
        };
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
                let opacity = try!(read_num::<Opacity>(value));
                layer.set_opacity(opacity);
            }
            "visibility" => {
                let visibility = try!(read_num::<u32>(value));
                if visibility == 0 {
                    layer.set_visibility(false);
                }
            }
            "offsetx" => {
                let offset_x = try!(read_num::<i32>(value));
                layer.set_offset_x(offset_x);
            }
            "offsety" => {
                let offset_y = try!(read_num::<i32>(value));
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
                let opacity = try!(read_num::<Opacity>(value));
                image_layer.set_opacity(opacity);
            }
            "visibility" => {
                let visibility = try!(read_num::<u32>(value));
                if visibility == 0 {
                    image_layer.set_visibility(false);
                }
            }
            "offsetx" => {
                let offset_x = try!(read_num::<i32>(value));
                image_layer.set_offset_x(offset_x);
            }
            "offsety" => {
                let offset_y = try!(read_num::<i32>(value));
                image_layer.set_offset_y(offset_y);
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

impl<R: Read> ElementReader<Image> for TmxReader<R> {
    fn read_attributes(&mut self, image: &mut Image, name: &str, value: &str) -> ::Result<()> {
        match name {
            "source" => {
                image.set_source(value);
            }
            "width" => {
                let width = try!(read_num(value));
                image.set_width(width);
            }
            "height" => {
                let height = try!(read_num(value));
                image.set_height(height);
            }
            _ => {
                return Err(Error::UnknownAttribute(name.to_string()));
            }
        };
        Ok(())
    }
}

impl<R: Read> ElementReader<TileOffset> for TmxReader<R> {
    fn read_attributes(&mut self, tile_offset: &mut TileOffset, name: &str, value: &str) -> ::Result<()> {
        match name {
            "x" => {
                let x = try!(read_num::<i32>(value));
                tile_offset.set_x(x);
            }
            "y" => {
                let y = try!(read_num::<i32>(value));
                tile_offset.set_y(y);
            }
            _ => {
                return Err(Error::UnknownAttribute(name.to_string()));
            }
        };
        Ok(())
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

impl<R: Read> ElementReader<Terrain> for TmxReader<R> {
    fn read_attributes(&mut self, terrain: &mut Terrain, name: &str, value: &str) -> ::Result<()> {
        match name {
            "name" => {
                terrain.set_name(value);
            }
            "tile" => {
                terrain.set_tile(value);
            }
            _ => {
                return Err(Error::UnknownAttribute(name.to_string()));
            }
        };
        Ok(())
    }

    fn read_children(&mut self, terrain: &mut Terrain, name: &str, attributes: &[OwnedAttribute]) -> ::Result<()>{
        match name {
            "properties" => {
                let properties = try!(self.on_properties(attributes));
                terrain.set_properties(properties);
            }
            _ => {}
        };
        Ok(())
    }
}

impl<R: Read> ElementReader<Tile> for TmxReader<R> {
    fn read_attributes(&mut self, tile: &mut Tile, name: &str, value: &str) -> ::Result<()> {
        match name {
            "id" => {
                let id = try!(read_num(value));
                tile.set_id(id);
            }
            _ => {
                return Err(Error::UnknownAttribute(name.to_string()));
            }
        };
        Ok(())
    }

    fn read_children(&mut self, tile: &mut Tile, name: &str, attributes: &[OwnedAttribute]) -> ::Result<()>{
        match name {
            "animation" => {
                let animation = try!(self.on_animation(attributes));
                tile.set_animation(animation);
            }
            "image" => {
                let image = try!(self.on_image(attributes));
                tile.set_image(image);
            }
            "objectgroup" => {
                let object_group = try!(self.on_object_group(attributes));
                tile.set_object_group(object_group);
            }
            "properties" => {
                let properties = try!(self.on_properties(attributes));
                tile.set_properties(properties);
            }
            _ => {}
        };
        Ok(())
    }
}

impl<R: Read> ElementReader<PropertySet> for TmxReader<R> {
    fn read_children(&mut self, properties: &mut PropertySet, name: &str, attributes: &[OwnedAttribute]) -> ::Result<()>{
        if name == "property" {
            let property = try!(self.on_property(attributes));
            properties.push(property);
        }
        Ok(())
    }
}

impl<R: Read> ElementReader<TerrainSet> for TmxReader<R> {
    fn read_children(&mut self, terrain_types: &mut TerrainSet, name: &str, attributes: &[OwnedAttribute]) -> ::Result<()>{
        if name == "terrain" {
            let terrain = try!(self.on_terrain(attributes));
            terrain_types.push(terrain);
        }
        Ok(())
    }
}

impl<R: Read> ElementReader<ObjectGroup> for TmxReader<R> {
    fn read_attributes(&mut self, object_group: &mut ObjectGroup, name: &str, value: &str) -> ::Result<()> {
        match name {
            "name" => {
                object_group.set_name(value);
            }
            "opacity" => {
                let opacity = try!(read_num::<Opacity>(value));
                object_group.set_opacity(opacity);
            }
            "visibility" => {
                let visibility = try!(read_num::<u32>(value));
                if visibility == 0 {
                    object_group.set_visibility(false);
                }
            }
            "offsetx" => {
                let offset_x = try!(read_num::<i32>(value));
                object_group.set_offset_x(offset_x);
            }
            "offsety" => {
                let offset_y = try!(read_num::<i32>(value));
                object_group.set_offset_y(offset_y);
            }
            "draworder" => {
                let draw_order = try!(DrawOrder::from_str(value));
                object_group.set_draw_order(draw_order);
            }
            _ => {
                return Err(Error::UnknownAttribute(name.to_string()));
            }
        };
        Ok(())
    }

    fn read_children(&mut self, object_group: &mut ObjectGroup, name: &str, attributes: &[OwnedAttribute]) -> ::Result<()>{
        match name {
            "properties" => {
                let properties = try!(self.on_properties(attributes));
                object_group.set_properties(properties);
            }
            _ => {}
        };
        Ok(())
    }
}

impl<R: Read> ElementReader<Animation> for TmxReader<R> {
    fn read_children(&mut self, animation: &mut Animation, name: &str, attributes: &[OwnedAttribute]) -> ::Result<()>{
        match name {
            "frame" => {
                let frame = try!(self.on_frame(attributes));
                animation.set_frame(frame);
            }
            _ => {}
        };
        Ok(())
    }
}

impl<R: Read> ElementReader<Frame> for TmxReader<R> {
    fn read_attributes(&mut self, frame: &mut Frame, name: &str, value: &str) -> ::Result<()> {
        match name {
            "duration" => {
                let duration = try!(read_num(value));
                frame.set_duration(duration);
            }
            "tileid" => {
                let tile_id = try!(read_num(value));
                frame.set_tile_id(tile_id);
            }
            _ => {
                return Err(Error::UnknownAttribute(name.to_string()));
            }
        };
        Ok(())
    }
}
