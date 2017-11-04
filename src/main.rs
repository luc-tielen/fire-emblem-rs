
#![feature(proc_macro)]
#![feature(refcell_replace_swap)]

extern crate cairo;
extern crate gdk;
extern crate gdk_pixbuf;
extern crate gtk;
extern crate futures;
extern crate futures_glib;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;
extern crate relm_attributes;


mod tileset;
mod tile;
mod gui_helpers;
mod map;
mod map_drawing_area;
mod tile_chooser;
mod gui;

use relm::Widget;
use gui::GUI;
use tileset::Tileset;


fn main() {
    let tileset_path = "./resources/tileset_castle_blue_green.png";
    let tile_width = 16;
    let tile_height = 16;
    let map_width = 20;
    let map_height = 20;

    let tileset = Tileset::new(tileset_path, tile_width, tile_height);
    let tiles = tileset.slice();
    GUI::run((tiles, map_width, map_height)).unwrap();
}
