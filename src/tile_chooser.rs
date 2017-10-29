
use relm::Widget;
use relm_attributes::widget;
use gtk;
use gtk::{WidgetExt, BoxExt};
use gdk;
use gdk::ContextExt;
use gdk::EventButton;
use cairo;
use std::rc::Rc;
use std::cell::Cell;
use std::iter::Iterator;
use self::TileChooserMsg::*;
use gui_helpers::MousePos;
use tile::Tile;


#[derive(Msg)]
pub enum TileChooserMsg {
    // index to clicked tile, can be out of bounds:
    LeftMouseClicked(Option<usize>),
    ChooserResized(u32, u32),  // width, height
}


#[derive(Debug, Clone, Copy)]
struct TileMetadata {
    num_tiles: usize,
    tile_width: u32,
    tile_height: u32,
}


pub struct Model {
    selected_tile: usize,  // tile index
    tiles: Vec<Tile>,
    tile_data: Rc<Cell<TileMetadata>>,
    chooser_width: Rc<Cell<u32>>,
    chooser_height: u32,
}


impl TileChooser {
    fn draw_tile_chooser(&self) {
        let width = self.get_map_width();
        let height = self.get_map_height();
        let tile_width = self.model.tile_data.get().tile_width;
        let tiles_per_row = width / tile_width;

        let ctx = self.get_drawing_context();
        ctx.rectangle(0.0, 0.0, width as f64, height as f64);
        ctx.fill();

        for (tile_idx, tile) in self.model.tiles.iter().enumerate() {
            self.draw_tile(&ctx, tile_idx as u32, tile, tiles_per_row);
        }
    }

    fn draw_tile(&self, ctx: &cairo::Context,
                 tile_idx: u32, tile: &Tile,
                 tiles_per_row: u32) {
        let row = tile_idx / tiles_per_row;
        let col = tile_idx % tiles_per_row;
        let x = col * tile.tile_width;
        let y = row * tile.tile_height;
        ctx.set_source_pixbuf(&tile.img, x as f64, y as f64);
        ctx.paint();
    }

    fn get_drawing_context(&self) -> cairo::Context {
        let canvas = &self.chooser;
        let window = &canvas.get_window().unwrap();
        cairo::Context::create_from_window(window)
    }

    fn get_map_width(&self) -> u32 {
        self.chooser.get_allocated_width() as u32
    }

    fn get_map_height(&self) -> u32 {
        self.chooser.get_allocated_height() as u32
    }
}

#[widget]
impl Widget for TileChooser {
    fn model(tiles: Vec<Tile>) -> Model {
        let tile_data = {
            let first_tile = &tiles[0];
            TileMetadata {
                num_tiles: tiles.len(),
                tile_width: first_tile.tile_width,
                tile_height: first_tile.tile_height,
            }
        };

        Model {
            selected_tile: 0,
            tiles: tiles,
            tile_data: Rc::new(Cell::new(tile_data)),
            chooser_width: Rc::new(Cell::new(0)),
            chooser_height: 0,
        }
    }

    fn update(&mut self, event: TileChooserMsg) {
        match event {
            LeftMouseClicked(maybe_tile_idx) => {
                if let Some(tile_idx) = maybe_tile_idx {
                    self.model.selected_tile = tile_idx;
                }
                self.draw_tile_chooser();
            },
            ChooserResized(width, height) => {
                self.model.chooser_width.set(width);
                self.model.chooser_height = height;
            },
        }
    }

    view! {
        #[name="chooser"]
        gtk::DrawingArea {
            can_focus: true,
            events: gdk::BUTTON_PRESS_MASK.bits() as i32,
            packing: {
                expand: true,
                fill: true,
            },
            size_allocate(_, sizes) =>
                ChooserResized(sizes.width as u32, sizes.height as u32),
            button_press_event(_, ev) with(tile_data, chooser_width)
                => (send_click_cmd(chooser_width.clone(),
                                   tile_data.clone(), ev),
                    gtk::Inhibit(false)),
        },
    }
}


fn send_click_cmd(rc_chooser_width: Rc<Cell<u32>>,
                  rc_tile_data: Rc<Cell<TileMetadata>>,
                  ev: &EventButton) -> TileChooserMsg {
    let mouse_pos = ev.get_position();
    let pos = MousePos {
        x: mouse_pos.0,
        y: mouse_pos.1,
    };

    let chooser_width = rc_chooser_width.get();
    let tile_data = rc_tile_data.get();
    let tiles_per_row = chooser_width / tile_data.tile_width;
    let sel_col = pos.x as u32 / tile_data.tile_height;
    let sel_row = pos.y as u32 / tile_data.tile_height;
    let tile_idx = (sel_row * tiles_per_row + sel_col) as usize;
    if tile_idx >= tile_data.num_tiles {
        LeftMouseClicked(None)
    }
    else {
        LeftMouseClicked(Some(tile_idx))
    }
}

