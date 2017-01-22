use std::path::Path;
use std::fs::File;

use color::Color;

mod reader;

macro_rules! define_iterator_wrapper {
    ($name: ident, $item: ident) => {
        pub struct $name<'a>(::std::slice::Iter<'a, $item>);

        impl<'a> Iterator for $name<'a> {
            type Item = &'a $item;

            fn next(&mut self) -> Option<Self::Item> {
                self.0.next()
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct Map {
    bg_color: Option<Color>,
    version: String,
    orientation: Orientation,
    render_order: RenderOrder,
    width: u32,
    height: u32,
    tile_width: u32,
    tile_height: u32,
    hex_side_length: Option<u32>,
    stagger_axis: Option<Axis>,
    stagger_index: Option<Index>,
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

    pub fn background_color(&self) -> Option<&Color> {
        self.bg_color.as_ref()
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

    pub fn hex_side_length(&self) -> Option<u32> {
        self.hex_side_length
    }

    pub fn stagger_axis(&self) -> Option<Axis> {
        self.stagger_axis
    }

    pub fn stagger_index(&self) -> Option<Index> {
        self.stagger_index
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

    fn set_background_color(&mut self, color: Color) {
        self.bg_color = Some(color);
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

    fn set_hex_side_length(&mut self, hex_side_length: u32) {
        self.hex_side_length = Some(hex_side_length);
    }

    fn set_stagger_axis(&mut self, stagger_axis: Axis) {
        self.stagger_axis = Some(stagger_axis);
    }

    fn set_stagger_index(&mut self, stagger_index: Index) {
        self.stagger_index = Some(stagger_index);
    }

    fn set_next_object_id(&mut self, next_object_id: u32) {
        self.next_object_id = next_object_id;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Index {
    Even,
    Odd,
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

define_iterator_wrapper!(Tilesets, Tileset);

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
struct PropertyCollection(Vec<Property>);

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

define_iterator_wrapper!(Properties, Property);

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
    properties: PropertyCollection,
    terrain_types: TerrainCollection,
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

#[derive(Debug, Default)]
pub struct Image {
    format: String,
    source: String,
    width: u32,
    height: u32,
}

impl Image {
    pub fn format(&self) -> &str {
        &self.format
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    fn set_format<S: Into<String>>(&mut self, format: S) {
        self.format = format.into();
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
    visible: bool,
    offset_x: i32,
    offset_y: i32,
    properties: PropertyCollection,
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
            visible: true,
            offset_x: 0,
            offset_y: 0,
            properties: PropertyCollection::new(),
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
        self.visible
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

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    fn set_offset_x(&mut self, offset_x: i32) {
        self.offset_x = offset_x;
    }

    fn set_offset_y(&mut self, offset_y: i32) {
        self.offset_y = offset_y;
    }

    fn set_properties(&mut self, properties: PropertyCollection) {
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

define_iterator_wrapper!(Layers, Layer);

#[derive(Debug)]
pub struct ImageLayer {
    name: String,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    opacity: Opacity,
    visible: bool,
    offset_x: i32,
    offset_y: i32,
    properties: PropertyCollection,
    image: Option<Image>,
}

impl Default for ImageLayer {
    fn default() -> ImageLayer {
        ImageLayer {
            name: String::default(),
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            opacity: 1.0,
            visible: true,
            offset_x: 0,
            offset_y: 0,
            properties: PropertyCollection::new(),
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
        self.visible
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

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
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

    fn set_properties(&mut self, properties: PropertyCollection) {
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

define_iterator_wrapper!(ImageLayers, ImageLayer);

pub type Opacity = f64;

#[derive(Debug)]
pub struct ObjectGroup {
    name: String,
    color: Option<Color>,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    opacity: Opacity,
    visible: bool,
    offset_x: i32,
    offset_y: i32,
    draw_order: DrawOrder,
    properties: PropertyCollection,
    objects: Vec<Object>,
}

impl ObjectGroup {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn color(&self) -> Option<&Color> {
        self.color.as_ref()
    }

    pub fn opacity(&self) -> Opacity {
        self.opacity
    }

    pub fn is_visible(&self) -> bool {
        self.visible
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

    pub fn objects(&self) -> Objects {
        Objects(self.objects.iter())
    }

    fn set_name<S: Into<String>>(&mut self, name: S) {
        self.name = name.into();
    }

    fn set_color(&mut self, color: Color) {
        self.color = Some(color);
    }

    fn set_opacity(&mut self, opacity: Opacity) {
        self.opacity = opacity;
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
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

    fn set_properties(&mut self, properties: PropertyCollection) {
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

    fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }
}

impl Default for ObjectGroup {
    fn default() -> ObjectGroup {
        ObjectGroup {
            name: String::default(),
            color: None,
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            opacity: 1.0,
            visible: true,
            offset_x: 0,
            offset_y: 0,
            draw_order: DrawOrder::TopDown,
            properties: PropertyCollection::new(),
            objects: Vec::new(),
        }
    }
}

define_iterator_wrapper!(ObjectGroups, ObjectGroup);

#[derive(Debug)]
pub struct Object {
    id: u32,
    name: String,
    object_type: String,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    rotation: f32,
    visible: bool,
    gid: Option<u32>,
    properties: PropertyCollection,
    shape: Option<Shape>,
}

impl Default for Object {
    fn default() -> Object {
        Object {
            id: 0,
            name: String::new(),
            object_type: String::new(),
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            rotation: 0.0,
            visible: true,
            gid: None,
            properties: PropertyCollection::new(),
            shape: None,
        }
    }
}

impl Object {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn object_type(&self) -> &str {
        &self.object_type
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

    pub fn rotation(&self) -> f32 {
        self.rotation
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn gid(&self) -> Option<u32> {
        self.gid
    }

    pub fn properties(&self) -> Properties {
        self.properties.iter()
    }

    pub fn shape(&self) -> Option<&Shape> {
        self.shape.as_ref()
    }

    fn set_id(&mut self, id: u32) {
        self.id = id;
    }

    fn set_name<S: Into<String>>(&mut self, name: S) {
        self.name = name.into();
    }

    fn set_object_type<S: Into<String>>(&mut self, object_type: S) {
        self.object_type = object_type.into();
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

    fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    fn set_gid(&mut self, gid: u32) {
        self.gid = Some(gid);
    }

    fn set_properties(&mut self, properties: PropertyCollection) {
        self.properties = properties;
    }

    fn set_shape<S: Into<Shape>>(&mut self, shape: S) {
        self.shape = Some(shape.into());
    }
}

define_iterator_wrapper!(Objects, Object);

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

define_iterator_wrapper!(Points, Point);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DrawOrder {
    TopDown,
    Index,
}

define_iterator_wrapper!(TerrainTypes, Terrain);

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

#[derive(Debug, Default)]
struct TerrainCollection(Vec<Terrain>);

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

