use std::collections::HashMap;

use crate::world::CHUNK_SIZE;


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ChunkPosition (pub i8, pub i8, pub i8);

impl ChunkPosition {
    pub fn is_valid(&self) -> bool {
        self.0 >= 0 && self.1 >= 0 && self.2 >= 0 && self.0 < CHUNK_SIZE && self.1 < CHUNK_SIZE && self.2 < CHUNK_SIZE
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    pub name: String,
}

#[derive(Debug, Clone, Default)]
pub struct Chunk {
    pub blocks: HashMap<ChunkPosition, Block>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            ..Default::default()
        }
    }

    pub fn add_block(&mut self, position : ChunkPosition, block : String) {
        if !position.is_valid() {
            return;
        }
        self.blocks.insert(position, Block {
            name: block,
        });
    }
}
