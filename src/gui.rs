
use relm::Widget;
use relm_attributes::widget;
use gtk;
use gtk::{WidgetExt, BoxExt, OrientableExt};
use gtk::Orientation::{Horizontal};
use gdk;
use gdk::prelude::{ContextExt};
use gdk::EventMotion;
use cairo;
use self::Msg::*;
use tile::Tile;


struct MousePos {
    x: f64,
    y: f64,
}

pub struct Model {
    loaded_tiles: Vec<Tile>,
}

#[derive(Msg)]
pub enum Msg {
    DrawMap(MousePos),
    Quit,
}


// TODO turn drawing area into widget!
impl GUI {
    fn draw_map(&self, pos: MousePos) {
        let width = self.get_map_width() as f64;
        let height = self.get_map_height() as f64;
        let ctx = self.get_drawing_context();
        ctx.rectangle(0.0, 0.0, width, height);
        ctx.fill();

        let tile = &self.model.loaded_tiles[0];
        ctx.set_source_pixbuf(&tile.img, pos.x, pos.y);
        ctx.paint();
        // TODO figure out how to pass params to GUI
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


fn send_draw_cmd(ev: &EventMotion) -> Msg {
    let pos = ev.get_position();
    let mouse_pos = MousePos {
        x: pos.0,
        y: pos.1,
    };
    DrawMap(mouse_pos)
}


#[widget]
impl Widget for GUI {
    fn model(tiles: Vec<Tile>) -> Model {
        Model {
            loaded_tiles: tiles,
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            DrawMap(pos) => {
                // TODO move mousepos into model?
                self.draw_map(pos);
            },
            Quit => gtk::main_quit(),
        }
    }

    view! {
        gtk::Window {
            gtk::Box {
                orientation: Horizontal,
                #[name="map_area"]
                gtk::DrawingArea {
                    can_focus: true,
                    events: gdk::POINTER_MOTION_MASK.bits() as i32,
                    packing: {
                        expand: true,
                        fill: true,
                    },
                    motion_notify_event(_, ev) => (send_draw_cmd(ev), gtk::Inhibit(false)),
                },
            },
            // TODO add status bar at bottom.
            delete_event(_, _) => (Quit, gtk::Inhibit(false)),
        }
    }
}
