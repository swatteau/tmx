#![allow(non_snake_case)]

extern crate tmx;

#[macro_use]
extern crate assert_matches;

#[test]
fn Map_NonExistentFile_ReturnsIoError() {
    let result = tmx::Map::open("non_existent_file.tmx");
    assert_matches!(result, Err(tmx::Error::Io(..)));
}

#[test]
fn Map_ValidXmlFile_ReturnsMap() {
    let result = tmx::Map::open("data/empty_map.tmx");
    assert_matches!(result, Ok(tmx::Map {..}));
}

#[test]
fn Tileset_NonExistentFile_ReturnsIoError() {
    let result = tmx::Tileset::open("non_existent_file.tsx");
    assert_matches!(result, Err(tmx::Error::Io(..)));
}

#[test]
fn Tileset_ValidXmlFile_ReturnsTileset() {
    let result = tmx::Tileset::open("data/simple_tileset.tsx");
    assert_matches!(result, Ok(tmx::Tileset {..}));
    let tileset = result.unwrap();
    let image = tileset.image().unwrap();
    assert_eq!(image.width(), 480);
    assert_eq!(image.height(), 480);
}

