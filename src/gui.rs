
use relm::Widget;
use relm_attributes::widget;
use gtk;
use gtk::{WidgetExt, OrientableExt};
use gtk::Orientation::{Horizontal};
use self::Msg::*;
use tile::Tile;
use map_drawing_area::MapDrawingArea;
use map_drawing_area::MapDrawingAreaMsg::TileSelected;
use tile_chooser::{TileChooser};
use tile_chooser::TileChooserMsg::LeftMouseClicked as TileChosen;


pub struct Model {
    loaded_tiles: Vec<Tile>,
}

#[derive(Msg)]
pub enum Msg {
    ChooserSelectedTile(Option<usize>),  // TODO rename
    Quit,
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
            ChooserSelectedTile(maybe_tile_idx) => {
                if let Some(tile_idx) = maybe_tile_idx {
                    let tile = self.model.loaded_tiles[tile_idx].clone();
                    let child_event = TileSelected(tile);
                    self.map_area.emit(child_event);
                }
            },
            Quit => gtk::main_quit(),
        }
    }

    view! {
        gtk::Window {
            gtk::Box {
                orientation: Horizontal,
                #[name="map_area"]
                MapDrawingArea(self.model.loaded_tiles[0].clone()) {},
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
