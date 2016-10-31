extern crate tmx;

#[macro_use]
extern crate assert_matches;

#[test]
fn when_reading_nonexistent_map_file_expect_io_error() {
    let result = tmx::Map::open("non_existent_file.tmx");
    assert_matches!(result, Err(tmx::Error::Io(..)));
}

#[test]
fn after_reading_valid_tmx_file_expect_valid_map() {
    let result = tmx::Map::open("data/empty_map.tmx");
    assert_matches!(result, Ok(tmx::Map {..}));
}

#[test]
fn when_reading_nonexistent_tileset_file_expect_io_error() {
    let result = tmx::Tileset::open("non_existent_file.tsx");
    assert_matches!(result, Err(tmx::Error::Io(..)));
}

#[test]
fn after_reading_valid_tsx_file_expect_valid_tileset() {
    let result = tmx::Tileset::open("data/simple_tileset.tsx");
    assert_matches!(result, Ok(tmx::Tileset {..}));
    let tileset = result.unwrap();
    let image = tileset.image().unwrap();
    assert_eq!(image.width(), 480);
    assert_eq!(image.height(), 480);
}

