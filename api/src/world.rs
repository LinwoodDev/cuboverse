use std::{collections::{HashMap, hash_map::Entry}, ops::Div};
use crate::chunk::*;

pub const CHUNK_SIZE: i8 = 16;

#[derive(Debug, Clone, Copy,Hash, PartialEq, Eq)]
pub struct ChunkLocation(pub i32,pub i32, pub i32);
#[derive(Debug, Clone)]
pub struct EntityLocation(pub f32, pub f32, pub f32);


#[derive(Debug, Clone)]
pub struct Item {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Inventory {
    pub hand : Option<Item>,
    pub items : Vec<Item>,
}

#[derive(Debug, Clone, Default)]
pub struct World {
    pub chunks: HashMap<ChunkLocation, Chunk>,
    pub entities: HashMap<ChunkLocation, Vec<Entity>>,
}

impl World {
    pub fn get_chunk(&mut self, location : ChunkLocation) -> &mut Chunk {
        match self.chunks.entry(location) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => v.insert(Chunk::new()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Entity {
    pub position: ChunkPosition,
    pub name: String,
    pub health: i32,
    pub max_health: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct GlobalPosition(pub ChunkLocation, pub ChunkPosition);

impl GlobalPosition {
    pub fn new(x : i64, y : i64, z: i64) -> Self {
        let chunk_size = CHUNK_SIZE as i64;
        let location = ChunkLocation(
            x.div(chunk_size) as i32,
            y.div(chunk_size) as i32,
            z.div(chunk_size) as i32,
        );
        let position = ChunkPosition(
            (x % chunk_size) as i8,
            (y % chunk_size) as i8,
            (z % chunk_size) as i8,
        );
        return GlobalPosition(location, position);
    }

    pub fn global_position(&self) -> (i64, i64, i64) {
        (
            self.0.0 as i64 * CHUNK_SIZE as i64 + self.1.0 as i64,
            self.0.1 as i64 * CHUNK_SIZE as i64 + self.1.1 as i64,
            self.0.2 as i64 * CHUNK_SIZE as i64 + self.1.2 as i64,
        )
    }

    pub fn move_position(&self, x : i64, y : i64, z: i64) -> GlobalPosition {
        let current = self.global_position();
        GlobalPosition::new(current.0 + x, current.1 + y, current.2 + z)
    }
}
