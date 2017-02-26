use std::io::Read;
use std::str::FromStr;
use std::path::Path;
use std::fs::File;
use xml::attribute::OwnedAttribute;
use error::Error;
use super::reader::{TmxReader, ElementReader, read_num};
use model::map::{ObjectGroup};

use model::property::*;
use model::image::Image;

#[derive(Debug, Default)]
pub struct Tileset {
    first_gid: u32,
    name: String,
    tile_width: u32,
    tile_height: u32,
    spacing: u32,
    margin: u32,
    tile_count: u32,
    columns: u32,
    image: Option<Image>,
    tile_offset: Option<TileOffset>,
    properties: PropertyCollection,
    terrain_types: TerrainCollection,
    tiles: Vec<Tile>,
}

impl Tileset {
    pub fn open<P: AsRef<Path>>(path: P) -> ::Result<Tileset> {
        let file = try!(File::open(path));
        let mut reader = TmxReader::new(file);
        reader.read_tileset()
    }

    pub fn first_gid(&self) -> u32 {
        self.first_gid
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn spacing(&self) -> u32 {
        self.spacing
    }

    pub fn margin(&self) -> u32 {
        self.margin
    }

    pub fn tile_width(&self) -> u32 {
        self.tile_width
    }

    pub fn tile_height(&self) -> u32 {
        self.tile_height
    }

    pub fn tile_count(&self) -> u32 {
        self.tile_count
    }

    pub fn columns(&self) -> u32 {
        self.columns
    }

    pub fn image(&self) -> Option<&Image> {
        self.image.as_ref()
    }

    pub fn tile_offset(&self) -> Option<TileOffset> {
        self.tile_offset
    }

    pub fn properties(&self) -> Properties {
        self.properties.iter()
    }

    pub fn terrain_types(&self) -> TerrainTypes {
        self.terrain_types.iter()
    }

    pub fn tiles(&self) -> Tiles {
        Tiles(self.tiles.iter())
    }

    fn set_first_gid(&mut self, first_gid: u32) {
        self.first_gid = first_gid;
    }

    fn set_name<S: Into<String>>(&mut self, name: S) {
        self.name = name.into();
    }

    fn set_spacing(&mut self, spacing: u32) {
        self.spacing = spacing;
    }

    fn set_margin(&mut self, margin: u32) {
        self.margin = margin;
    }

    fn set_tile_width(&mut self, tile_width: u32) {
        self.tile_width = tile_width;
    }

    fn set_tile_height(&mut self, tile_height: u32) {
        self.tile_height = tile_height;
    }

    fn set_tile_count(&mut self, tile_count: u32) {
        self.tile_count = tile_count;
    }

    fn set_columns(&mut self, columns: u32) {
        self.columns = columns;
    }

    fn set_image(&mut self, image: Image) {
        self.image = Some(image);
    }

    fn set_tile_offset(&mut self, tile_offset: TileOffset) {
        self.tile_offset = Some(tile_offset);
    }

    fn set_properties(&mut self, properties: PropertyCollection) {
        self.properties = properties;
    }

    fn set_terrain_types(&mut self, terrain_types: TerrainCollection) {
        self.terrain_types = terrain_types;
    }

    fn add_tile(&mut self, tile: Tile) {
        self.tiles.push(tile);
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct TileOffset {
    x: i32,
    y: i32,
}

impl TileOffset {
    pub fn new(x: i32, y: i32) -> TileOffset {
        TileOffset {
            x: x,
            y: y,
        }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    fn set_y(&mut self, y: i32) {
        self.y = y;
    }
}

#[derive(Debug, Default)]
pub struct TerrainCollection(Vec<Terrain>);

impl TerrainCollection {
    pub fn iter(&self) -> TerrainTypes {
        TerrainTypes(self.0.iter())
    }

    pub fn push(&mut self, terrain: Terrain) {
        self.0.push(terrain);
    }
}

#[derive(Debug, Default)]
pub struct Tile {
    id: u32,
    corners: Option<Corners>,
    probability: Option<f32>,
    animation: Option<Animation>,
    image: Option<Image>,
    object_group: Option<ObjectGroup>,
    properties: PropertyCollection,
}

impl Tile {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn terrain(&self) -> Option<&Corners> {
        self.corners.as_ref()
    }

    pub fn probability(&self) -> Option<f32> {
        self.probability
    }

    pub fn animation(&self) -> Option<&Animation> {
        self.animation.as_ref()
    }

    pub fn image(&self) -> Option<&Image> {
        self.image.as_ref()
    }

    pub fn object_group(&self) -> Option<&ObjectGroup> {
        self.object_group.as_ref()
    }

    pub fn properties(&self) -> Properties {
        self.properties.iter()
    }

    fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    fn set_corners(&mut self, corners: Corners) {
        self.corners = Some(corners);
    }

    fn set_probability(&mut self, probability: f32) {
        self.probability = Some(probability);
    }

    fn set_animation(&mut self, animation: Animation) {
        self.animation = Some(animation);
    }

    fn set_image(&mut self, image: Image) {
        self.image = Some(image);
    }

    fn set_object_group(&mut self, object_group: ObjectGroup) {
        self.object_group = Some(object_group);
    }

    fn set_properties(&mut self, properties: PropertyCollection) {
        self.properties = properties;
    }
}

define_iterator_wrapper!(Tiles, Tile);

#[derive(Debug, PartialEq, Eq)]
pub struct Corners(pub u32, pub u32, pub u32, pub u32);

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

#[derive(Debug, Default)]
pub struct Animation {
    frame: Option<Frame>,
}

impl Animation {
    pub fn frame(&self) -> Option<&Frame> {
        self.frame.as_ref()
    }

    fn set_frame(&mut self, frame: Frame) {
        self.frame = Some(frame);
    }
}

#[derive(Debug, Default)]
pub struct Frame {
    duration: u32,
    tile_id: u32,
}

impl Frame {
    pub fn duration(&self) -> u32 {
        self.duration
    }

    pub fn tile_id(&self) -> u32 {
        self.tile_id
    }

    fn set_duration(&mut self, duration: u32) {
        self.duration = duration;
    }

    fn set_tile_id(&mut self, tile_id: u32) {
        self.tile_id = tile_id;
    }
}

#[derive(Debug, Default)]
pub struct Terrain {
    name: String,
    tile: String,
    properties: PropertyCollection,
}

impl Terrain {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn tile(&self) -> &str {
        &self.tile
    }

    pub fn properties(&self) -> Properties {
        self.properties.iter()
    }

    fn set_name<S: Into<String>>(&mut self, name: S) {
        self.name = name.into();
    }

    fn set_tile<S: Into<String>>(&mut self, tile: S) {
        self.tile = tile.into();
    }

    fn set_properties(&mut self, properties: PropertyCollection) {
        self.properties = properties;
    }

}

define_iterator_wrapper!(TerrainTypes, Terrain);

impl FromStr for Tileset {
    type Err = Error;

    fn from_str(s: &str) -> ::Result<Tileset> {
        let mut tsx = TmxReader::new(s.as_bytes());
        tsx.read_tileset()
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

impl<R: Read> ElementReader<TileOffset> for TmxReader<R> {
    fn read_attributes(&mut self, tile_offset: &mut TileOffset, name: &str, value: &str) -> ::Result<()> {
        match name {
            "x" => {
                let x = try!(read_num(value));
                tile_offset.set_x(x);
            }
            "y" => {
                let y = try!(read_num(value));
                tile_offset.set_y(y);
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
            "terrain" => {
                let corners = try!(Corners::from_str(value));
                tile.set_corners(corners);
            }
            "probability" => {
                let probability = try!(read_num(value));
                if probability < 0.0 || probability > 1.0 {
                    return Err(Error::BadProbability(probability));
                }
                tile.set_probability(probability);
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

impl<R: Read> ElementReader<PropertyCollection> for TmxReader<R> {
    fn read_children(&mut self, properties: &mut PropertyCollection, name: &str, attributes: &[OwnedAttribute]) -> ::Result<()>{
        if name == "property" {
            let property = try!(self.on_property(attributes));
            properties.push(property);
        }
        Ok(())
    }
}

impl<R: Read> ElementReader<TerrainCollection> for TmxReader<R> {
    fn read_children(&mut self, terrain_types: &mut TerrainCollection, name: &str, attributes: &[OwnedAttribute]) -> ::Result<()>{
        if name == "terrain" {
            let terrain = try!(self.on_terrain(attributes));
            terrain_types.push(terrain);
        }
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

