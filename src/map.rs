use std::cmp::{min, max};
use std::fmt::Display;
use rltk::{RGB, Rltk, BLACK};
use serde::{Deserialize, Serialize};
use specs::{Join, World, WorldExt};
use super::{BelongsTo, Impassable, Position, Rect};

pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 43;
pub const MAP_TILES: i32 = MAP_WIDTH * MAP_HEIGHT;

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum TileType {
    Wall,
    Floor,
    RequiresKey,
    Portal,
}

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum Place {
    School,
    Library,
}

impl Place {
    pub fn get_name(&self) -> String {
        String::from(match self {
            Place::School => "Okul",
            Place::Library => "Kütüphane",
        })
    }
    pub fn get_year(&self) -> String {
        String::from(match self {
            Place::School => "2023",
            Place::Library => "2022",
        })
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub width: i32,
    pub height: i32,
    pub place: Place,
}

impl Map {
    pub fn xy_to_tile(x: i32, y: i32) -> usize {
        (y * MAP_WIDTH + x) as usize
    }
    fn adjust_tiles(&mut self, ecs: &mut World) {
        let current_place = ecs.fetch::<Place>();
        let impassables = ecs.read_storage::<Impassable>();
        let positions = ecs.read_storage::<Position>();
        let belongs = ecs.read_storage::<BelongsTo>();
        for (_imp, pos, bel) in (&impassables, &positions, &belongs).join() {
            if bel.domain == *current_place {
                self.tiles[Map::xy_to_tile(pos.x, pos.y)] = TileType::RequiresKey;
            }
        }
    }

    /// Takes a room, in the form of a rect, and alters the map accordingly to project the room
    fn apply_room_to_map(&mut self, room: &Rect) {
        for x in room.x1 + 1..=room.x2 {
            for y in room.y1 + 1..=room.y2 {
                let index = Map::xy_to_tile(x, y);
                if index > 0 && index < self.width as usize * self.height as usize {
                    self.tiles[index] = TileType::Floor;
                }
            }
        }
    }
    pub fn new_map_rooms_and_corridors(ecs: &mut World, place: Place) -> Map {
        let mut map = Map {
            tiles: vec![TileType::Wall; MAP_TILES as usize],
            width: MAP_WIDTH,
            height: MAP_HEIGHT,
            place,
        };

        match place {
            Place::School => {
                let school = Rect::new(4, 4, 30, 20);
                map.apply_room_to_map(&school);
            }
            Place::Library => {
                let library = Rect::new(10, 10, 30, 30);
                map.apply_room_to_map(&library);
            }
        }

        map.adjust_tiles(ecs);

        map
    }
}

pub fn draw_map(ecs: &World, ctx: &mut Rltk) {
    let map = ecs.fetch::<Map>();

    let mut x: i32 = 0;
    let mut y: i32 = 0;

    for tile in map.tiles.iter() {
        match tile {
            TileType::Wall => {
                ctx.set(x, y, RGB::from_u8(0, 127, 0), RGB::named(BLACK), rltk::to_cp437('#'));
            }
            TileType::Floor => {
                ctx.set(x, y, RGB::from_u8(0, 63, 0), RGB::named(BLACK), rltk::to_cp437('.'));
            }
            _ => {}
        }
        x += 1;
        if x >= MAP_WIDTH {
            x = 0;
            y += 1;
        }
    }
}

