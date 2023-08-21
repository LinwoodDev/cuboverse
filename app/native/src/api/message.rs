use super::*;
use flutter_rust_bridge::frb;

#[frb(mirror(ChunkPosition))]
pub struct VisualChunkPosition(pub i8, pub i8, pub i8);
#[frb(mirror(ChunkLocation))]
pub struct VisualChunkLocation(pub i32, pub i32, pub i32);
#[frb(mirror(GlobalPosition))]
pub struct VisualGlobalPosition(pub ChunkLocation, pub ChunkPosition);

pub struct BlockInformation {
    pub name : String,
    pub position : ChunkPosition,
}

pub enum NativeMessage {
    AddBlock {
        chunk : ChunkLocation,
        block : BlockInformation,
    },
    RemoveBlock {
        position : ChunkPosition,
        chunk : ChunkLocation,
    },
    AddChunk {
        location : ChunkLocation,
        blocks : Vec<BlockInformation>,
    },
    RemoveChunk {
        location : ChunkLocation,
    },
    PlayerTeleported(GlobalPosition),
}
