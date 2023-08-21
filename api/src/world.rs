use std::collections::HashMap;
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
