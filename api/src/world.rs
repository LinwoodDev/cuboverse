use std::collections::HashMap;
use crate::{chunk::*, entity::Entity, physics::RigidBody, block::GlobalBlockPosition};

pub const CHUNK_SIZE: i8 = 16;


#[derive(Debug, Clone, Copy,Hash, PartialEq, Eq)]
pub struct ChunkLocation(pub i32,pub i32, pub i32);

impl ChunkLocation {
    pub fn distance(&self, other :&ChunkLocation) -> u32 {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1) + self.2.abs_diff(other.2)
    }
}


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
    pub fn new() -> Self {
        World {
            ..Default::default()
        }
    }
    pub fn get_chunk(&mut self, location : ChunkLocation) -> &mut Chunk {
        self.chunks.entry(location).or_insert_with(|| Chunk::new())
    }

    pub fn add_entity(&mut self, entity : Entity, location : ChunkLocation) {
        let chunk = self.entities.entry(location).or_insert_with(|| Vec::new());
        chunk.push(entity);
    }

    pub fn move_entity(&mut self, entity : &Entity, old : ChunkLocation, new : ChunkLocation) {
        if old == new {
            return;
        }
        let entities = &mut self.entities;
        let old_chunk = entities.get_mut(&old);
        let Some(old_chunk) = old_chunk else {
            return;
        };
        let index = old_chunk.iter().position(|e| e == entity);
        let Some(index) = index else {
            return;
        };
        let entity = old_chunk.remove(index);
        let new_chunk = entities.entry(new).or_insert_with(|| Vec::new());
        new_chunk.push(entity);
    }

    pub fn tick(&mut self) {
        let entities = self.entities.clone();
        for (chunk_location, mut entities) in entities {
            // Clone the chunk location for the tick call
            let cloned_chunk_location = chunk_location.clone();

            // Iterate through entities associated with the current chunk location
            for entity in entities.iter_mut() {
                entity.tick(self, cloned_chunk_location.clone());
            }
        }
    }

    pub fn has_block(&self, position : GlobalBlockPosition) -> bool {
        let chunk = self.chunks.get(&position.0);
        let Some(chunk) = chunk else {
            return false;
        };
        chunk.has_block(position.1)
    }
}

pub trait ChunkGenerator {
    fn generate_chunk(&self, location : ChunkLocation) -> Chunk;
}

