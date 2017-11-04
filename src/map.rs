
use std::ops::Index;
use std::ops::IndexMut;
use tile::Tile;


pub type MapIdx = u16;
pub type MapSize = u16;

#[derive(Debug, Clone)]
pub struct MapLocation {
    pub x: MapIdx,
    pub y: MapIdx,
}


#[derive(Debug, Clone)]
pub struct Map {
    tiles: Vec<Option<Tile>>,  // width * height long
    pub width: MapSize,
    pub height: MapSize,
}

impl Map {
    pub fn new(width: MapSize, height: MapSize) -> Self {
        Map {
            tiles: vec![None; (width * height) as usize],
            width: width,
            height: height,
        }
    }

    pub fn set_tile(&mut self, tile: Tile, loc: MapLocation) {
        if loc.x >= self.width || loc.y >= self.height { return; }
        self[loc] = Some(tile)
    }

    pub fn get_tile<'a>(&'a self, loc: MapLocation) -> &'a Option<Tile> {
        &self[loc]
    }

    pub fn clear_tile(&mut self, loc: MapLocation) {
        if loc.x >= self.width || loc.y >= self.height { return; }
        self[loc] = None
    }
}


pub struct MapIterator<'a> {
    map: &'a Map,
    curr_idx: usize,
}

impl<'a> MapIterator<'a> {
    fn new(map: &'a Map) -> Self {
        MapIterator {
            map: map,
            curr_idx: 0,
        }
    }
}

impl<'a> IntoIterator for &'a Map {
    type Item = (MapLocation, &'a Option<Tile>);
    type IntoIter = MapIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        MapIterator::new(self)
    }
}

impl<'a> Iterator for MapIterator<'a> {
    type Item = (MapLocation, &'a Option<Tile>);

    fn next(&mut self) -> Option<Self::Item> {
        let curr_idx = self.curr_idx;
        if curr_idx == (self.map.width * self.map.height) as usize {
            None
        }
        else {
            let row = curr_idx / self.map.width as usize;
            let col = curr_idx % self.map.width as usize;
            let loc = MapLocation { x: col as u16, y: row as u16 };
            let item = &self.map[loc.clone()];
            self.curr_idx += 1;
            Some((loc, item))
        }
    }
}


impl Index<MapIdx> for Map {
    type Output = [Option<Tile>];

    fn index<'a>(&'a self, idx: MapIdx) -> &'a [Option<Tile>] {
        let idx_begin = idx as usize;
        let idx_end = idx_begin + self.width as usize;
        &self.tiles[idx_begin..idx_end]
    }
}

impl Index<(MapIdx, MapIdx)> for Map {
    type Output = Option<Tile>;

    fn index<'a>(&'a self, idx: (MapIdx, MapIdx)) -> &'a Option<Tile> {
        let elem_idx = (idx.1 * self.width + idx.0) as usize;
        &self.tiles[elem_idx]
    }
}

impl Index<MapLocation> for Map {
    type Output = Option<Tile>;

    fn index<'a>(&'a self, idx: MapLocation) -> &'a Option<Tile> {
        &self[(idx.x, idx.y)]
    }
}

impl IndexMut<MapIdx> for Map {
    fn index_mut<'a>(&'a mut self, idx: MapIdx) -> &'a mut [Option<Tile>] {
        let idx_begin = idx as usize;
        let idx_end = idx_begin + self.width as usize;
        &mut self.tiles[idx_begin..idx_end]
    }
}

impl IndexMut<(MapIdx, MapIdx)> for Map {
    fn index_mut<'a>(&'a mut self, idx: (MapIdx, MapIdx)) -> &'a mut Option<Tile> {
        let elem_idx = (idx.1 * self.width + idx.0) as usize;
        &mut self.tiles[elem_idx]
    }
}

impl IndexMut<MapLocation> for Map {
    fn index_mut<'a>(&'a mut self, idx: MapLocation) -> &'a mut Option<Tile> {
        &mut self[(idx.x, idx.y)]
    }
}
