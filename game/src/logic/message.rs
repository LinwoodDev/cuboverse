use super::*;
use flutter_rust_bridge::frb;

#[derive(Debug, Clone)]
#[frb(mirror(BlockPosition))]
pub struct VisualBlockPosition(pub i8, pub i8, pub i8);
#[derive(Debug, Clone)]
#[frb(mirror(EntityPosition))]
pub struct VisualEntityPosition(pub f32, pub f32, pub f32);
#[derive(Debug, Clone)]
#[frb(mirror(ChunkLocation))]
pub struct VisualChunkLocation(pub i32, pub i32, pub i32);
#[derive(Debug, Clone)]
#[frb(mirror(GlobalBlockPosition))]
pub struct VisualGlobalBlockPosition(pub ChunkLocation, pub BlockPosition);
#[derive(Debug, Clone)]
#[frb(mirror(GlobalEntityPosition))]
pub struct VisualGlobalEntityPosition(pub ChunkLocation, pub EntityPosition);

#[derive(Debug, Clone)]
pub struct BlockInformation {
    pub name: String,
    pub position: BlockPosition,
}

#[derive(Debug, Clone)]
pub enum NativeMessage {
    AddBlock {
        chunk: ChunkLocation,
        block: BlockInformation,
    },
    RemoveBlock {
        position: BlockPosition,
        chunk: ChunkLocation,
    },
    AddChunk {
        location: ChunkLocation,
        blocks: Vec<BlockInformation>,
    },
    RemoveChunk {
        location: ChunkLocation,
    },
    PlayerTeleported {
        x: f64,
        y: f64,
        z: f64,
    },
}

pub struct WorldMessenger(pub Option<StreamSink<NativeMessage>>);

pub(crate) fn to_block_information(position : BlockPosition, block : &Block) -> BlockInformation {
    BlockInformation {
        name: block.name.clone(),
        position,
    }
}

impl WorldMessenger {
    pub fn send_message(&self, message: NativeMessage) {
        if let Some(s) = &self.0 {
            s.add(message);
        }
    }
    pub(crate) fn send_player_teleported(&self, player: &Player) {
        let pos = player.position.global_position();
        self.send_message(NativeMessage::PlayerTeleported {
            x: pos.0,
            y: pos.1,
            z: pos.2,
        });
    }

    pub(crate) fn send_add_block(&self, position : GlobalBlockPosition, block: &Block) {
        self.send_message(NativeMessage::AddBlock { chunk: position.0, block: to_block_information(position.1, block) });
    }

    pub(crate) fn send_remove_block(&self, chunk: ChunkLocation, position: BlockPosition) {
        self.send_message(NativeMessage::RemoveBlock { chunk, position });
    }

    pub(crate) fn send_add_chunk(&self, location: ChunkLocation, chunk: &Chunk) {
        self.send_message(NativeMessage::AddChunk {
            location,
            blocks: chunk
                .get_blocks()
                .iter()
                .map(|(k, v)| to_block_information(*k, v))
                .collect(),
        });
    }

    pub(crate) fn send_remove_chunk(&self, location: ChunkLocation) {
        self.send_message(NativeMessage::RemoveChunk { location });
    }
}
