use api::{chunk::ChunkPosition, world::ChunkLocation};

enum NativeMessage {
    AddBlock {
        name : String,
        position : ChunkPosition,
        chunk : ChunkLocation,
    },
    RemoveBlock {
        position : ChunkPosition,
        chunk : ChunkLocation,
    }
}
