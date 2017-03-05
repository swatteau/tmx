extern crate tmx;

use std::env;
use std::error::Error;
use std::fmt::Debug;

use tmx::{Map, Tileset};

fn show_usage() {
    println!("Usage: {0} <file>", env::args().nth(0).unwrap());
    println!("  where <file> is the path to a .tmx or .tsx file");
}

fn dump<T: Debug, E: Error>(result: &Result<T, E>) {
    match *result {
        Ok(ref t) => println!("{:#?}", t),
        Err(ref e) => println!("Error: {}", e)
    };
}

fn main() {
    if let Some(path) = env::args().nth(1) {
        if path.ends_with(".tmx") {
            dump(&Map::open(path));
        } else if path.ends_with(".tsx") {
            dump(&Tileset::open(path));
        } else {
            println!("Error: a .tmx or .tsx file is expected.");
        }
    } else {
        show_usage();
    }
}
