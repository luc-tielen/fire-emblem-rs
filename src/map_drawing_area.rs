
use relm::Widget;
use relm_attributes::widget;
use gtk;
use gtk::{WidgetExt, BoxExt};
use gdk;
use gdk::prelude::{ContextExt};
use gdk::EventButton;
use cairo;
use gui_helpers::MousePos;
use self::MapDrawingAreaMsg::*;
use tile::Tile;


struct Map {
    // TODO
}

impl Map {
    fn new() -> Self {
        Map {

        }
    }
}

pub struct Model {
    selected_tile: Tile,
    map: Map,
}


#[derive(Msg)]
pub enum MapDrawingAreaMsg {
    LeftMouseClicked(MousePos),
    TileSelected(Tile),
}


impl MapDrawingArea {
    fn draw_map(&self, pos: MousePos) {
        let width = self.get_map_width() as f64;
        let height = self.get_map_height() as f64;
        let ctx = self.get_drawing_context();
        ctx.rectangle(0.0, 0.0, width, height);
        ctx.fill();

        let tile = &self.model.selected_tile;
        ctx.set_source_pixbuf(&tile.img, pos.x, pos.y);
        ctx.paint();
    }

    fn get_drawing_context(&self) -> cairo::Context {
        let canvas = &self.map_area;
        let window = &canvas.get_window().unwrap();
        cairo::Context::create_from_window(window)
    }

    fn get_map_width(&self) -> u32 {
        self.map_area.get_allocated_width() as u32
    }

    fn get_map_height(&self) -> u32 {
        self.map_area.get_allocated_height() as u32
    }
}


fn send_draw_cmd(ev: &EventButton) -> MapDrawingAreaMsg {
    let pos = ev.get_position();
    let mouse_pos = MousePos {
        x: pos.0,
        y: pos.1,
    };
    LeftMouseClicked(mouse_pos)
}


#[widget]
impl Widget for MapDrawingArea {
    fn model(tile: Tile) -> Model {
        Model {
            selected_tile: tile,
            map: Map::new(),
        }
    }

    fn update(&mut self, event: MapDrawingAreaMsg) {
        match event {
            LeftMouseClicked(pos) => {
                self.draw_map(pos);
            },
            TileSelected(tile) => {
                self.model.selected_tile = tile;
            },
        }
    }

    view! {
        #[name="map_area"]
        gtk::DrawingArea {
            can_focus: true,
            events: gdk::BUTTON_PRESS_MASK.bits() as i32,
            packing: {
                expand: true,
                fill: true,
            },
            button_press_event(_, ev) => (send_draw_cmd(ev), gtk::Inhibit(false)),
        }
    }
}
