use crate::{
    entity::{EntityPosition, GlobalEntityPosition},
    world::{ChunkLocation, CHUNK_SIZE},
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct BlockPosition(pub u8, pub u8, pub u8);

impl BlockPosition {
    pub fn is_valid(&self) -> bool {
        self.0 < CHUNK_SIZE && self.1 < CHUNK_SIZE && self.2 < CHUNK_SIZE
    }

    pub fn get_entity_position(&self) -> EntityPosition {
        EntityPosition(self.0 as f32, self.1 as f32, self.2 as f32)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GlobalBlockPosition(pub ChunkLocation, pub BlockPosition);

impl GlobalBlockPosition {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        let chunk_size = CHUNK_SIZE as i64;
        let location = ChunkLocation(
            (x / chunk_size) as i32 + if x < 0 { -1 } else { 0 },
            (y / chunk_size) as i32 + if y < 0 { -1 } else { 0 },
            (z / chunk_size) as i32 + if z < 0 { -1 } else { 0 },
        );
        let position = BlockPosition(
            (x % chunk_size) as u8 + if x < 0 { chunk_size as u8 } else { 0 },
            (y % chunk_size) as u8 + if y < 0 { chunk_size as u8 } else { 0 },
            (z % chunk_size) as u8 + if z < 0 { chunk_size as u8 } else { 0 },
        );
        GlobalBlockPosition(location, position)
    }

    pub fn global_position(&self) -> (i64, i64, i64) {
        (
            self.0 .0 as i64 * CHUNK_SIZE as i64 + self.1 .0 as i64,
            self.0 .1 as i64 * CHUNK_SIZE as i64 + self.1 .1 as i64,
            self.0 .2 as i64 * CHUNK_SIZE as i64 + self.1 .2 as i64,
        )
    }

    pub fn get_entity_position(&self) -> GlobalEntityPosition {
        GlobalEntityPosition(self.0, self.1.get_entity_position())
    }

    pub fn move_position(&self, x: i64, y: i64, z: i64) -> GlobalBlockPosition {
        let current = self.global_position();
        GlobalBlockPosition::new(current.0 + x, current.1 + y, current.2 + z)
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    pub name: String,
}
