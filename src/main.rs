
/*
extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;
*/

extern crate image;


mod tileset;
mod tile;

use tileset::Tileset;
use std::path::Path;


fn main() {
    let tileset_path = Path::new("./resources/tileset_castle_blue_green.png");
    let tile_width = 16;
    let tile_height = 16;

    let tileset = Tileset::new(tileset_path, tile_width, tile_height);
    let tiles = tileset.slice();
    println!("{:?}", tiles[0])
}
