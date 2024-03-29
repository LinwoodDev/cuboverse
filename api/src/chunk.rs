use std::collections::HashMap;

use crate::block::{BlockPosition, Block};


#[derive(Debug, Clone, Default)]
pub struct Chunk {
    blocks: HashMap<BlockPosition, Block>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            ..Default::default()
        }
    }

    pub fn get_blocks(&self) -> &HashMap<BlockPosition, Block> {
        &self.blocks
    }

    pub fn add_block(&mut self, position : BlockPosition, block : String) -> Option<Block> {
        if !position.is_valid() {
            return None;
        }
        self.blocks.insert(position, Block {
            name: block,
        })
    }

    pub fn remove_block(&mut self, position : &BlockPosition) -> Option<Block> {
        self.blocks.remove(position)
    }

    pub fn has_block(&self, block_position: BlockPosition) -> bool {
        self.blocks.contains_key(&block_position)
    }
}
