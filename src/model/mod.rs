use std::path::Path;
use std::fs::File;

mod reader;

#[derive(Debug, Default)]
pub struct Map {
    version: String,
    orientation: Orientation,
    render_order: RenderOrder,
    width: u32,
    height: u32,
    tile_width: u32,
    tile_height: u32,
    next_object_id: u32,
    tilesets: Vec<Tileset>,
    layers: Vec<Layer>,
    image_layers: Vec<ImageLayer>,
    object_groups: Vec<ObjectGroup>,
}

impl Map {
    pub fn open<P: AsRef<Path>>(path: P) -> ::Result<Map> {
        let file = try!(File::open(path));
        let mut reader = reader::TmxReader::new(file);
        reader.read_map()
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn orientation(&self) -> Orientation {
        self.orientation
    }

    pub fn render_order(&self) -> RenderOrder {
        self.render_order
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn tile_width(&self) -> u32 {
        self.tile_width
    }

    pub fn tile_height(&self) -> u32 {
        self.tile_height
    }

    pub fn next_object_id(&self) -> u32 {
        self.next_object_id
    }

    fn add_tileset(&mut self, tileset: Tileset) {
        self.tilesets.push(tileset);
    }

    fn add_layer(&mut self, layer: Layer) {
        self.layers.push(layer);
    }

    fn add_image_layer(&mut self, image_layer: ImageLayer) {
        self.image_layers.push(image_layer);
    }

    fn add_object_group(&mut self, object_group: ObjectGroup) {
        self.object_groups.push(object_group);
    }

    pub fn tilesets(&self) -> Tilesets {
        Tilesets(self.tilesets.iter())
    }

    pub fn layers(&self) -> Layers {
        Layers(self.layers.iter())
    }

    pub fn image_layers(&self) -> ImageLayers {
        ImageLayers(self.image_layers.iter())
    }

    pub fn object_groups(&self) -> ObjectGroups {
        ObjectGroups(self.object_groups.iter())
    }

    fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    fn set_orientation(&mut self, orientation: Orientation) {
        self.orientation = orientation;
    }

    fn set_render_order(&mut self, render_order: RenderOrder) {
        self.render_order = render_order;
    }

    fn set_version<S: Into<String>>(&mut self, version: S) {
        self.version = version.into();
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn set_tile_width(&mut self, tile_width: u32) {
        self.tile_width = tile_width;
    }

    fn set_tile_height(&mut self, tile_height: u32) {
        self.tile_height = tile_height;
    }

    fn set_next_object_id(&mut self, next_object_id: u32) {
        self.next_object_id = next_object_id;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Orientation {
    Orthogonal,
    Isometric,
    Staggered,
    Hexagonal,
}

impl Default for Orientation {
    fn default() -> Orientation {
        Orientation::Orthogonal
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderOrder {
    RightDown,
    RightUp,
    LeftDown,
    LeftUp,
}

impl Default for RenderOrder {
    fn default() -> RenderOrder {
        RenderOrder::RightDown
    }
}

pub struct Tilesets<'a>(::std::slice::Iter<'a, Tileset>);

impl<'a> Iterator for Tilesets<'a> {
    type Item = &'a Tileset;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

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

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn property_type(&self) -> PropertyType {
        self.property_type
    }

    fn set_name<S: Into<String>>(&mut self, name: S) {
        self.name = name.into();
    }

    fn set_value<S: Into<String>>(&mut self, value: S) {
        self.value = value.into();
    }

    fn set_property_type(&mut self, property_type: PropertyType) {
        self.property_type = property_type;
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
pub struct PropertySet(Vec<Property>);

impl PropertySet {
    pub fn new() -> PropertySet {
        PropertySet(Vec::new())
    }

    pub fn push(&mut self, property: Property) {
        self.0.push(property);
    }

    pub fn iter(&self) -> Properties {
        Properties(self.0.iter())
    }
}

pub struct Properties<'a>(::std::slice::Iter<'a, Property>);

impl<'a> Iterator for Properties<'a> {
    type Item = &'a Property;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
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
    properties: PropertySet,
    terrain_types: TerrainSet,
    tiles: Vec<Tile>,
}

impl Tileset {
    pub fn open<P: AsRef<Path>>(path: P) -> ::Result<Tileset> {
        let file = try!(File::open(path));
        let mut reader = reader::TmxReader::new(file);
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

    fn set_properties(&mut self, properties: PropertySet) {
        self.properties = properties;
    }

    fn set_terrain_types(&mut self, terrain_types: TerrainSet) {
        self.terrain_types = terrain_types;
    }

    fn add_tile(&mut self, tile: Tile) {
        self.tiles.push(tile);
    }
}

#[derive(Debug, Default)]
pub struct Image {
    source: String,
    width: u32,
    height: u32,
}

impl Image {
    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    fn set_source<S: Into<String>>(&mut self, source: S) {
        self.source = source.into();
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn set_height(&mut self, height: u32) {
        self.height = height;
    }
}

#[derive(Debug)]
pub struct Layer {
    name: String,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    opacity: Opacity,
    visibility: bool,
    offset_x: i32,
    offset_y: i32,
    properties: PropertySet,
}

impl Default for Layer {
    fn default() -> Layer {
        Layer {
            name: String::default(),
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            opacity: 1.0,
            visibility: true,
            offset_x: 0,
            offset_y: 0,
            properties: PropertySet::new(),
        }
    }
}

impl Layer {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn opacity(&self) -> Opacity {
        self.opacity
    }

    pub fn is_visible(&self) -> bool {
        self.visibility
    }

    pub fn offset_x(&self) -> i32 {
        self.offset_x
    }

    pub fn offset_y(&self) -> i32 {
        self.offset_y
    }

    pub fn properties(&self) -> Properties {
        self.properties.iter()
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    fn set_name<S: Into<String>>(&mut self, name: S) {
        self.name = name.into();
    }

    fn set_opacity(&mut self, opacity: Opacity) {
        self.opacity = opacity;
    }

    fn set_visibility(&mut self, visibility: bool) {
        self.visibility = visibility;
    }

    fn set_offset_x(&mut self, offset_x: i32) {
        self.offset_x = offset_x;
    }

    fn set_offset_y(&mut self, offset_y: i32) {
        self.offset_y = offset_y;
    }

    fn set_properties(&mut self, properties: PropertySet) {
        self.properties = properties;
    }

    fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn set_height(&mut self, height: u32) {
        self.height = height;
    }
}

pub struct Layers<'a>(::std::slice::Iter<'a, Layer>);

impl<'a> Iterator for Layers<'a> {
    type Item = &'a Layer;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

#[derive(Debug)]
pub struct ImageLayer {
    name: String,
    x: i32,
    y: i32,
    opacity: Opacity,
    visibility: bool,
    offset_x: i32,
    offset_y: i32,
    properties: PropertySet,
    image: Option<Image>,
}

impl Default for ImageLayer {
    fn default() -> ImageLayer {
        ImageLayer {
            name: String::default(),
            x: 0,
            y: 0,
            opacity: 1.0,
            visibility: true,
            offset_x: 0,
            offset_y: 0,
            properties: PropertySet::new(),
            image: None,
        }
    }
}

impl ImageLayer {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn opacity(&self) -> Opacity {
        self.opacity
    }

    pub fn is_visible(&self) -> bool {
        self.visibility
    }

    pub fn offset_x(&self) -> i32 {
        self.offset_x
    }

    pub fn offset_y(&self) -> i32 {
        self.offset_y
    }

    pub fn image(&self) -> Option<&Image> {
        self.image.as_ref()
    }

    pub fn properties(&self) -> Properties {
        self.properties.iter()
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    fn set_name<S: Into<String>>(&mut self, name: S) {
        self.name = name.into();
    }

    fn set_opacity(&mut self, opacity: Opacity) {
        self.opacity = opacity;
    }

    fn set_visibility(&mut self, visibility: bool) {
        self.visibility = visibility;
    }

    fn set_offset_x(&mut self, offset_x: i32) {
        self.offset_x = offset_x;
    }

    fn set_offset_y(&mut self, offset_y: i32) {
        self.offset_y = offset_y;
    }

    fn set_image(&mut self, image: Image) {
        self.image = Some(image);
    }

    fn set_properties(&mut self, properties: PropertySet) {
        self.properties = properties;
    }

    fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    fn set_y(&mut self, y: i32) {
        self.y = y;
    }
}

pub struct ImageLayers<'a>(::std::slice::Iter<'a, ImageLayer>);

impl<'a> Iterator for ImageLayers<'a> {
    type Item = &'a ImageLayer;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

pub type Opacity = f64;

#[derive(Debug)]
pub struct ObjectGroup {
    name: String,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    opacity: Opacity,
    visibility: bool,
    offset_x: i32,
    offset_y: i32,
    draw_order: DrawOrder,
    properties: PropertySet,
}

impl ObjectGroup {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn opacity(&self) -> Opacity {
        self.opacity
    }

    pub fn is_visible(&self) -> bool {
        self.visibility
    }

    pub fn draw_order(&self) -> DrawOrder {
        self.draw_order
    }

    pub fn offset_x(&self) -> i32 {
        self.offset_x
    }

    pub fn offset_y(&self) -> i32 {
        self.offset_y
    }

    pub fn properties(&self) -> Properties {
        self.properties.iter()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    fn set_name<S: Into<String>>(&mut self, name: S) {
        self.name = name.into();
    }

    fn set_opacity(&mut self, opacity: Opacity) {
        self.opacity = opacity;
    }

    fn set_visibility(&mut self, visibility: bool) {
        self.visibility = visibility;
    }

    fn set_draw_order(&mut self, draw_order: DrawOrder) {
        self.draw_order = draw_order;
    }

    fn set_offset_x(&mut self, offset_x: i32) {
        self.offset_x = offset_x;
    }

    fn set_offset_y(&mut self, offset_y: i32) {
        self.offset_y = offset_y;
    }

    fn set_properties(&mut self, properties: PropertySet) {
        self.properties = properties;
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    fn set_y(&mut self, y: i32) {
        self.y = y;
    }
}

impl Default for ObjectGroup {
    fn default() -> ObjectGroup {
        ObjectGroup {
            name: String::default(),
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            opacity: 1.0,
            visibility: true,
            offset_x: 0,
            offset_y: 0,
            draw_order: DrawOrder::TopDown,
            properties: PropertySet::new(),
        }
    }
}

pub struct ObjectGroups<'a>(::std::slice::Iter<'a, ObjectGroup>);

impl<'a> Iterator for ObjectGroups<'a> {
    type Item = &'a ObjectGroup;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawOrder {
    TopDown,
    Index,
}

pub struct TerrainTypes<'a>(::std::slice::Iter<'a, Terrain>);

impl<'a> Iterator for TerrainTypes<'a> {
    type Item = &'a Terrain;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

#[derive(Debug, Default)]
pub struct Terrain {
    name: String,
    tile: String,
    properties: PropertySet,
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

    fn set_properties(&mut self, properties: PropertySet) {
        self.properties = properties;
    }

}

#[derive(Debug, Default)]
pub struct TerrainSet(Vec<Terrain>);

impl TerrainSet {
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
    animation: Option<Animation>,
    image: Option<Image>,
    object_group: Option<ObjectGroup>,
    properties: PropertySet,
}

impl Tile {
    pub fn id(&self) -> u32 {
        self.id
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

    fn set_animation(&mut self, animation: Animation) {
        self.animation = Some(animation);
    }

    fn set_image(&mut self, image: Image) {
        self.image = Some(image);
    }

    fn set_object_group(&mut self, object_group: ObjectGroup) {
        self.object_group = Some(object_group);
    }

    fn set_properties(&mut self, properties: PropertySet) {
        self.properties = properties;
    }
}

pub struct Tiles<'a>(::std::slice::Iter<'a, Tile>);

impl<'a> Iterator for Tiles<'a> {
    type Item = &'a Tile;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
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

#[cfg(test)]
mod tests;

