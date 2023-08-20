use crate::world::*;

pub struct BlockTickEvent {
    pub block : Block,
    pub chunk_position : ChunkPosition,
    pub chunk : Chunk,
    pub world : World,
}

pub enum EntityTickEvent {
    Global {
        entity : Entity
    }
}

pub struct EventHandler {
    pub block_tick: Vec<BlockTickEvent>,
    pub entity_tick: Vec<EntityTickEvent>,
}
