use std::io::Read;
use xml::attribute::OwnedAttribute;
use error::Error;
use super::reader::{TmxReader, ElementReader, read_num};

#[derive(Debug, Default)]
pub struct Data {
    encoding: Option<String>,
    compression: Option<String>,
    raw: Option<String>,
    tiles: Vec<DataTile>,
}

impl Data {
    pub fn encoding(&self) -> Option<&str> {
        self.encoding.as_ref().map(String::as_str)
    }

    pub fn compression(&self) -> Option<&str> {
        self.compression.as_ref().map(String::as_str)
    }

    pub fn raw_content(&self) -> Option<&str> {
        self.raw.as_ref().map(String::as_str)
    }

    pub fn tiles(&self) -> DataTiles {
        DataTiles(self.tiles.iter())
    }

    fn set_encoding<S: Into<String>>(&mut self, encoding: S) {
        self.encoding = Some(encoding.into());
    }

    fn set_compression<S: Into<String>>(&mut self, compression: S) {
        self.compression = Some(compression.into());
    }

    fn set_raw_content<S: Into<String>>(&mut self, content: S) {
        self.raw = Some(content.into());
    }

    fn add_tile(&mut self, tile: DataTile) {
        self.tiles.push(tile);
    }
}

define_iterator_wrapper!(DataTiles, DataTile);

#[derive(Debug, Default)]
pub struct DataTile {
    gid: i32,
}

impl DataTile {
    fn set_gid(&mut self, gid: i32) {
        self.gid = gid;
    }
}

impl<R: Read> ElementReader<Data> for TmxReader<R> {
    fn read_attributes(&mut self, data: &mut Data, name: &str, value: &str) -> ::Result<()> {
        match name {
            "encoding" => {
                data.set_encoding(value);
            }
            "compression" => {
                data.set_compression(value);
            }
            _ => {
                return Err(Error::UnknownAttribute(name.to_string()));
            }
        };
        Ok(())
    }

    fn read_children(&mut self, data: &mut Data, name: &str, attributes: &[OwnedAttribute]) -> ::Result<()>{
        if name == "tile" {
            let tile = try!(self.on_data_tile(attributes));
            data.add_tile(tile);
        }
        Ok(())
    }

    fn read_content(&mut self, data: &mut Data, content: &str) -> ::Result<()> {
        data.set_raw_content(content);
        Ok(())
    }
}

impl<R: Read> ElementReader<DataTile> for TmxReader<R> {
    fn read_attributes(&mut self, tile: &mut DataTile, name: &str, value: &str) -> ::Result<()> {
        match name {
            "gid" => {
                let gid = try!(read_num(value));
                tile.set_gid(gid);
            }
            _ => {
                return Err(Error::UnknownAttribute(name.to_string()));
            }
        };
        Ok(())
    }
}

