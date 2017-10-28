

use std::cell::RefCell;
use image::{
    GenericImage,
    RgbaImage,
    Rgba,
};


#[derive(Debug)]
pub struct Tile {
    img: RgbaImage,
    tile_width: u32,
    tile_height: u32,
}


impl Tile {
    pub fn new(rc_tileset_img: &RefCell<RgbaImage>,
               tile_width: u32, tile_height: u32,
               row: u32, col: u32) -> Tile {
        let x = row * tile_width;
        let y = col * tile_height;
        let mut tileset_img = rc_tileset_img.borrow_mut();
        Tile {
            img: tileset_img.sub_image(x, y, tile_width, tile_height).to_image(),
            tile_width: tile_width,
            tile_height: tile_height,
        }
    }

    pub fn is_empty(&self) -> bool {
        let black_pixel_count = self.img
            .pixels()
            .filter(|&pxl| pxl == &Rgba([0, 0, 0, 0]))
            .count();
        let pixel_count = (self.tile_width * self.tile_height) as usize;
        black_pixel_count == pixel_count
    }
}
