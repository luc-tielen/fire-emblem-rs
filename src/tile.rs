
use gdk_pixbuf::Pixbuf;


#[derive(Debug, Clone)]
pub struct Tile {
    pub img: Pixbuf,
    pub tile_width: u32,
    pub tile_height: u32,
}


impl Tile {
    pub fn new(
        tileset_img: &Pixbuf,
        tile_width: u32,
        tile_height: u32,
        row: u32,
        col: u32,
    ) -> Tile {
        let x = row * tile_width;
        let y = col * tile_height;
        Tile {
            img: tileset_img.new_subpixbuf(
                x as i32,
                y as i32,
                tile_width as i32,
                tile_height as i32,
            ),
            tile_width: tile_width,
            tile_height: tile_height,
        }
    }

    pub fn is_empty(&self) -> bool {
        unsafe {
            let pxls = self.img.get_pixels();
            for byte in pxls {
                // if any of the pixels bytes is non-zero,
                // -> not completely black -> not empty
                if byte != &0 {
                    return false;
                }
            }
        }
        true
    }
}
