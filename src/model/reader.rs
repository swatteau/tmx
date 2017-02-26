use std::io::Read;
use std::str::FromStr;
use xml::reader::{EventReader, XmlEvent};
use xml::attribute::OwnedAttribute;

use error::Error;
use model::map::{ImageLayer, Layer, Map, Object, ObjectGroup, Polygon, Polyline};
use model::data::{Data, DataTile};
use model::property::{PropertyCollection, Property};
use model::image::Image;
use model::tileset::{Animation, Terrain, TerrainCollection, Tile, TileOffset, Tileset, Frame};

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

