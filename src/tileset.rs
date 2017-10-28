
use std::iter::repeat;
use std::path::Path;
use std::cell::RefCell;
use image;
use image::{
    RgbaImage,
};
use tile::Tile;


#[derive(Debug)]
pub struct Tileset {
    tiles: RefCell<RgbaImage>,
    tile_width: u32,
    tile_height: u32,
}


#[derive(Debug)]
pub struct Dimensions {
    width: u32,
    height: u32,
}


impl Tileset {
    pub fn new(tileset_path: &Path, tile_width: u32, tile_height: u32) -> Tileset {
        let img = image::open(tileset_path).unwrap();
        let tileset = Tileset {
            tiles: RefCell::new(img.to_rgba()),
            tile_width: tile_width,
            tile_height: tile_height,
        };
        assert!(tileset.dimensions().width % tile_width == 0);
        assert!(tileset.dimensions().height % tile_height == 0);
        tileset
    }

    pub fn dimensions(&self) -> Dimensions {
        let dim = self.tiles.borrow().dimensions();
        Dimensions {
            width: dim.0,
            height: dim.1,
        }
    }

    pub fn slice(& self) -> Vec<Tile> {
        let dim = self.dimensions();
        let num_rows = dim.width / self.tile_width;
        let num_columns = dim.height / self.tile_height;
        let rows = 0..(num_rows - 1);
        let columns = 0..(num_columns - 1);

        rows.flat_map(|row| repeat(row).zip(columns.clone()))
            .filter_map(|(row, col)| {
                let tile = Tile::new(&self.tiles, self.tile_width,
                                     self.tile_height, row, col);
                if tile.is_empty() {
                    None
                }
                else {
                  Some(tile)
                }
            })
            .collect()
    }
}


