use std::{collections::HashMap, ops::Div};
use crate::chunk::*;

pub const CHUNK_SIZE: i8 = 16;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
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

#[derive(Debug, Clone)]
pub struct Entity {
    pub position: ChunkPosition,
    pub name: String,
    pub health: i32,
    pub max_health: i32,
}

#[derive(Debug, Clone)]
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
}
