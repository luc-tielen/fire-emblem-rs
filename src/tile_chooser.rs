
use relm::Widget;
use relm_attributes::widget;
use gtk;
use gtk::{WidgetExt, BoxExt};
use gdk;
use gdk::EventButton;
use cairo;
use std::rc::Rc;
use std::cell::Cell;
use self::TileChooserMsg::*;
use gui_helpers::MousePos;
use tile::Tile;


#[derive(Msg)]
pub enum TileChooserMsg {
    LeftMouseClicked(usize),   // index to clicked tile
    ChooserResized(u32, u32),  // width, height
}


#[derive(Debug, Clone, Copy)]
struct TileMetadata {
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
    fn draw(&self) {
        let width = self.get_map_width();
        let height = self.get_map_height();
        let tiles_per_row = width / self.model.tile_width;


        // TODO draw.
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
            LeftMouseClicked(tile_idx) => {
                self.model.selected_tile = tile_idx;
                self.draw();
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
    let tile_width = &tile_data.tile_width;
    let tile_height = &tile_data.tile_height;
    let tiles_per_row = chooser_width / tile_width;
    let sel_col = pos.x as u32 / tile_height;
    let sel_row = (pos.y as u32 / tile_height) * tiles_per_row;
    let tile_idx = sel_row * tiles_per_row + sel_col;
    LeftMouseClicked(tile_idx as usize)
}

