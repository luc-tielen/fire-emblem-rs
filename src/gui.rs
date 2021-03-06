
use relm::{Relm, Widget};
use relm_attributes::widget;
use gtk;
use gtk::{WidgetExt, OrientableExt};
use gtk::Orientation::Horizontal;
use futures_glib::Interval;
use std::time::Duration;
use self::Msg::*;
use tile::Tile;
use map::MapSize;
use map_drawing_area::MapDrawingArea;
use map_drawing_area::MapDrawingAreaMsg::TileSelected;
use map_drawing_area::MapDrawingAreaMsg as MDAMsg;
use tile_chooser::TileChooser;
use tile_chooser::TileChooserMsg::LeftMouseClicked as TileChosen;
use tile_chooser::TileChooserMsg as TCMsg;


pub struct Model {
    loaded_tiles: Vec<Tile>,
    map_width: u16,
    map_height: u16,
}


#[derive(Msg)]
pub enum Msg {
    TimerExpired(()),
    ChooserSelectedTile(Option<usize>),
    Quit,
}


#[widget]
impl Widget for GUI {
    fn model(data: (Vec<Tile>, MapSize, MapSize)) -> Model {
        let tiles = data.0;
        let map_width = data.1;
        let map_height = data.2;
        Model {
            loaded_tiles: tiles,
            map_width: map_width,
            map_height: map_height,
        }
    }

    fn subscriptions(&mut self, relm: &Relm<Self>) {
        let stream = Interval::new(Duration::from_millis(500));
        relm.connect_exec_ignore_err(stream, TimerExpired);
    }

    fn update(&mut self, event: Msg) {
        match event {
            TimerExpired(_) => {
                self.tile_chooser.emit(TCMsg::TimerExpired);
                self.map_area.emit(MDAMsg::TimerExpired);
            }
            ChooserSelectedTile(maybe_tile_idx) => {
                if let Some(tile_idx) = maybe_tile_idx {
                    let tile = self.model.loaded_tiles[tile_idx].clone();
                    let child_event = TileSelected(tile);
                    self.map_area.emit(child_event);
                }
            }
            Quit => gtk::main_quit(),
        }
    }

    view! {
        gtk::Window {
            gtk::Box {
                orientation: Horizontal,
                #[name="map_area"]
                MapDrawingArea(self.model.loaded_tiles[0].clone(),
                               self.model.map_width,
                               self.model.map_height) {},
                #[name="tile_chooser"]
                TileChooser(self.model.loaded_tiles.clone()) {
                    TileChosen(tile_idx) => ChooserSelectedTile(tile_idx)
                },
            },
            // TODO add status bar at bottom.
            delete_event(_, _) => (Quit, gtk::Inhibit(false)),
        }
    }
}

