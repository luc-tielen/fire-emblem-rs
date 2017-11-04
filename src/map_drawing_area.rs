
use relm::Widget;
use relm_attributes::widget;
use gtk;
use gtk::{WidgetExt, BoxExt};
use gdk;
use gdk::prelude::ContextExt;
use gdk::EventButton;
use cairo::Context as Ctx;
use gui_helpers::MousePos;
use self::MapDrawingAreaMsg::*;
use tile::Tile;
use map::{Map, MapSize, MapLocation};


pub struct Model {
    selected_tile: Tile,
    map: Map,
}


#[derive(Msg)]
pub enum MapDrawingAreaMsg {
    TimerExpired,
    LeftMouseClicked(MousePos),
    RightMouseClicked(MousePos),
    TileSelected(Tile),
}


impl MapDrawingArea {
    fn draw_map(&self) {
        let width = self.get_map_width() as f64;
        let height = self.get_map_height() as f64;
        let ctx = self.get_drawing_context();
        self.draw_background(&ctx, width, height);
        self.draw_tiles(&ctx);
    }

    fn draw_background(&self, ctx: &Ctx, width: f64, height: f64) {
        ctx.rectangle(0.0, 0.0, width, height);
        ctx.fill();
    }

    fn draw_tiles(&self, ctx: &Ctx) {
        for (map_loc, maybe_tile) in &self.model.map {
            if let &Some(ref tile) = maybe_tile {
                let x = (map_loc.x as f64) * (tile.tile_height as f64);
                let y = (map_loc.y as f64) * (tile.tile_width as f64);
                ctx.set_source_pixbuf(&tile.img, x, y);
                ctx.paint();
            }
        }
    }

    fn get_drawing_context(&self) -> Ctx {
        let canvas = &self.map_area;
        let window = &canvas.get_window().unwrap();
        Ctx::create_from_window(window)
    }

    fn get_map_width(&self) -> u32 {
        self.map_area.get_allocated_width() as u32
    }

    fn get_map_height(&self) -> u32 {
        self.map_area.get_allocated_height() as u32
    }

    fn mouse_to_map_loc(&self, pos: MousePos) -> MapLocation {
        let map_x = pos.x as u16 / self.model.selected_tile.tile_width as u16;
        let map_y = pos.y as u16 / self.model.selected_tile.tile_height as u16;
        MapLocation { x: map_x, y: map_y }
    }
}


#[widget]
impl Widget for MapDrawingArea {
    fn model(data: (Tile, MapSize, MapSize)) -> Model {
        let tile = data.0;
        let map_width = data.1;
        let map_height = data.2;
        Model {
            selected_tile: tile,
            map: Map::new(map_width, map_height),
        }
    }

    fn update(&mut self, event: MapDrawingAreaMsg) {
        match event {
            TimerExpired => {
                let ctx = self.get_drawing_context();
                let width = self.get_map_width() as f64;
                let height = self.get_map_height() as f64;
                self.draw_background(&ctx, width, height);
                self.draw_map();
            },
            LeftMouseClicked(pos) => {
                let loc = self.mouse_to_map_loc(pos);
                self.model.map.set_tile(self.model.selected_tile.clone(), loc);
                self.draw_map();
            },
            RightMouseClicked(pos) => {
                let loc = self.mouse_to_map_loc(pos);
                self.model.map.clear_tile(loc);
                self.draw_map();
            },
            TileSelected(tile) => {
                self.model.selected_tile = tile;
            },
        }
    }

    view! {
        #[name="map_area"]
        gtk::DrawingArea {
            events: (gdk::BUTTON1_MASK | gdk::BUTTON3_MASK).bits() as i32,
            packing: {
                expand: true,
                fill: true,
            },
            button_press_event(_, ev) => (notify_mouse_click(ev), gtk::Inhibit(false)),
        }
    }
}


fn notify_mouse_click(ev: &EventButton) -> MapDrawingAreaMsg {
    let pos = ev.get_position();
    let mouse_pos = MousePos { x: pos.0, y: pos.1 };
    match ev.get_button() {
        1 => LeftMouseClicked(mouse_pos),
        3 => RightMouseClicked(mouse_pos),
        _ => RightMouseClicked(mouse_pos),
    }
}
