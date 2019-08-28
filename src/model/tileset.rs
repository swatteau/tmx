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
use std::path::Path;
use std::fs::File;

use xml::attribute::OwnedAttribute;

use error::Error;
use model::image::Image;
use model::map::ObjectGroup;
use model::property::{Properties, PropertyCollection};
use model::reader::{self, TmxReader, ElementReader};

define_iterator_wrapper!(Tiles, Tile);
define_iterator_wrapper!(TerrainTypes, Terrain);

#[derive(Debug, Default)]
pub struct Tileset {
    first_gid: u32,
    source: String,
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
        let file = File::open(path)?;
        let mut reader = TmxReader::new(file);
        reader.read_tileset()
    }

    pub fn first_gid(&self) -> u32 {
        self.first_gid
    }

    fn set_first_gid(&mut self, first_gid: u32) {
        self.first_gid = first_gid;
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    fn set_source<S: Into<String>>(&mut self, source: S) {
        self.source = source.into();
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    fn set_name<S: Into<String>>(&mut self, name: S) {
        self.name = name.into();
    }

    pub fn tile_width(&self) -> u32 {
        self.tile_width
    }

    fn set_tile_width(&mut self, tile_width: u32) {
        self.tile_width = tile_width;
    }

    pub fn tile_height(&self) -> u32 {
        self.tile_height
    }

    fn set_tile_height(&mut self, tile_height: u32) {
        self.tile_height = tile_height;
    }

    pub fn spacing(&self) -> u32 {
        self.spacing
    }

    fn set_spacing(&mut self, spacing: u32) {
        self.spacing = spacing;
    }

    pub fn margin(&self) -> u32 {
        self.margin
    }

    fn set_margin(&mut self, margin: u32) {
        self.margin = margin;
    }

    pub fn tile_count(&self) -> u32 {
        self.tile_count
    }

    fn set_tile_count(&mut self, tile_count: u32) {
        self.tile_count = tile_count;
    }

    pub fn columns(&self) -> u32 {
        self.columns
    }

    fn set_columns(&mut self, columns: u32) {
        self.columns = columns;
    }

    pub fn tile_offset(&self) -> Option<TileOffset> {
        self.tile_offset
    }

    fn set_tile_offset(&mut self, tile_offset: TileOffset) {
        self.tile_offset = Some(tile_offset);
    }

    pub fn properties(&self) -> Properties {
        self.properties.iter()
    }

    fn set_properties(&mut self, properties: PropertyCollection) {
        self.properties = properties;
    }

    pub fn image(&self) -> Option<&Image> {
        self.image.as_ref()
    }

    fn set_image(&mut self, image: Image) {
        self.image = Some(image);
    }

    pub fn terrain_types(&self) -> TerrainTypes {
        self.terrain_types.iter()
    }

    fn set_terrain_types(&mut self, terrain_types: TerrainCollection) {
        self.terrain_types = terrain_types;
    }

    pub fn tiles(&self) -> Tiles {
        Tiles(self.tiles.iter())
    }

    fn add_tile(&mut self, tile: Tile) {
        self.tiles.push(tile);
    }
}

impl FromStr for Tileset {
    type Err = Error;

    fn from_str(s: &str) -> ::Result<Tileset> {
        let mut tsx = TmxReader::new(s.as_bytes());
        tsx.read_tileset()
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

    fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    fn set_y(&mut self, y: i32) {
        self.y = y;
    }
}

#[derive(Debug, Default)]
pub struct TerrainCollection(Vec<Terrain>);

impl TerrainCollection {
    fn iter(&self) -> TerrainTypes {
        TerrainTypes(self.0.iter())
    }

    fn push(&mut self, terrain: Terrain) {
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

    fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    pub fn terrain(&self) -> Option<&Corners> {
        self.corners.as_ref()
    }

    fn set_corners(&mut self, corners: Corners) {
        self.corners = Some(corners);
    }

    pub fn probability(&self) -> Option<f32> {
        self.probability
    }

    fn set_probability(&mut self, probability: f32) {
        self.probability = Some(probability);
    }

    pub fn properties(&self) -> Properties {
        self.properties.iter()
    }

    fn set_properties(&mut self, properties: PropertyCollection) {
        self.properties = properties;
    }

    pub fn image(&self) -> Option<&Image> {
        self.image.as_ref()
    }

    fn set_image(&mut self, image: Image) {
        self.image = Some(image);
    }

    pub fn object_group(&self) -> Option<&ObjectGroup> {
        self.object_group.as_ref()
    }

    fn set_object_group(&mut self, object_group: ObjectGroup) {
        self.object_group = Some(object_group);
    }

    pub fn animation(&self) -> Option<&Animation> {
        self.animation.as_ref()
    }

    fn set_animation(&mut self, animation: Animation) {
        self.animation = Some(animation);
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Corners(pub u32, pub u32, pub u32, pub u32);

impl FromStr for Corners {
    type Err = Error;

    fn from_str(s: &str) -> ::Result<Corners> {
        let ids = {
            let result: Result<Vec<u32>, _> = s.split(',').map(reader::read_num).collect();
            result?
        };
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
    pub fn tile_id(&self) -> u32 {
        self.tile_id
    }

    fn set_tile_id(&mut self, tile_id: u32) {
        self.tile_id = tile_id;
    }

    pub fn duration(&self) -> u32 {
        self.duration
    }

    fn set_duration(&mut self, duration: u32) {
        self.duration = duration;
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

    fn set_name<S: Into<String>>(&mut self, name: S) {
        self.name = name.into();
    }

    pub fn tile(&self) -> &str {
        &self.tile
    }

    fn set_tile<S: Into<String>>(&mut self, tile: S) {
        self.tile = tile.into();
    }

    pub fn properties(&self) -> Properties {
        self.properties.iter()
    }

    fn set_properties(&mut self, properties: PropertyCollection) {
        self.properties = properties;
    }
}

impl<R: Read> ElementReader<Tileset> for TmxReader<R> {
    fn read_attributes(&mut self, tileset: &mut Tileset, name: &str, value: &str) -> ::Result<()> {
        match name {
            "firstgid" => {
                let first_gid = reader::read_num(value)?;
                tileset.set_first_gid(first_gid);
            }
            "source" => {
                tileset.set_source(value);
            }
            "name" => {
                tileset.set_name(value);
            }
            "tilewidth" => {
                let tile_width = reader::read_num(value)?;
                tileset.set_tile_width(tile_width);
            }
            "tileheight" => {
                let tile_height = reader::read_num(value)?;
                tileset.set_tile_height(tile_height);
            }
            "spacing" => {
                let spacing = reader::read_num(value)?;
                tileset.set_spacing(spacing);
            }
            "margin" => {
                let margin = reader::read_num(value)?;
                tileset.set_margin(margin);
            }
            "tilecount" => {
                let tile_count = reader::read_num(value)?;
                tileset.set_tile_count(tile_count);
            }
            "columns" => {
                let columns = reader::read_num(value)?;
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
            "tileoffset" => {
                let tile_offset = self.on_tile_offset(attributes)?;
                tileset.set_tile_offset(tile_offset);
            }
            "properties" => {
                let properties = self.on_properties(attributes)?;
                tileset.set_properties(properties);
            }
            "image" => {
                let image = self.on_image(attributes)?;
                tileset.set_image(image);
            }
            "terraintypes" => {
                let terrain_types = self.on_terrain_types(attributes)?;
                tileset.set_terrain_types(terrain_types);
            }
            "tile" => {
                let tile = self.on_tile(attributes)?;
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
                let x = reader::read_num(value)?;
                tile_offset.set_x(x);
            }
            "y" => {
                let y = reader::read_num(value)?;
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
        if let "properties" = name {
            let properties = self.on_properties(attributes)?;
            terrain.set_properties(properties);
        }
        Ok(())
    }
}

impl<R: Read> ElementReader<Tile> for TmxReader<R> {
    fn read_attributes(&mut self, tile: &mut Tile, name: &str, value: &str) -> ::Result<()> {
        match name {
            "id" => {
                let id = reader::read_num(value)?;
                tile.set_id(id);
            }
            "terrain" => {
                let corners = Corners::from_str(value)?;
                tile.set_corners(corners);
            }
            "probability" => {
                let probability = reader::read_num(value)?;
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
            "properties" => {
                let properties = self.on_properties(attributes)?;
                tile.set_properties(properties);
            }
            "image" => {
                let image = self.on_image(attributes)?;
                tile.set_image(image);
            }
            "objectgroup" => {
                let object_group = self.on_object_group(attributes)?;
                tile.set_object_group(object_group);
            }
            "animation" => {
                let animation = self.on_animation(attributes)?;
                tile.set_animation(animation);
            }
            _ => {}
        };
        Ok(())
    }
}

impl<R: Read> ElementReader<PropertyCollection> for TmxReader<R> {
    fn read_children(&mut self, properties: &mut PropertyCollection, name: &str, attributes: &[OwnedAttribute]) -> ::Result<()>{
        if name == "property" {
            let property = self.on_property(attributes)?;
            properties.push(property);
        }
        Ok(())
    }
}

impl<R: Read> ElementReader<TerrainCollection> for TmxReader<R> {
    fn read_children(&mut self, terrain_types: &mut TerrainCollection, name: &str, attributes: &[OwnedAttribute]) -> ::Result<()>{
        if name == "terrain" {
            let terrain = self.on_terrain(attributes)?;
            terrain_types.push(terrain);
        }
        Ok(())
    }
}

impl<R: Read> ElementReader<Animation> for TmxReader<R> {
    fn read_children(&mut self, animation: &mut Animation, name: &str, attributes: &[OwnedAttribute]) -> ::Result<()>{
        if let "frame" = name {
            let frame = self.on_frame(attributes)?;
            animation.set_frame(frame);
        }
        Ok(())
    }
}

impl<R: Read> ElementReader<Frame> for TmxReader<R> {
    fn read_attributes(&mut self, frame: &mut Frame, name: &str, value: &str) -> ::Result<()> {
        match name {
            "tileid" => {
                let tile_id = reader::read_num(value)?;
                frame.set_tile_id(tile_id);
            }
            "duration" => {
                let duration = reader::read_num(value)?;
                frame.set_duration(duration);
            }
            _ => {
                return Err(Error::UnknownAttribute(name.to_string()));
            }
        };
        Ok(())
    }
}

