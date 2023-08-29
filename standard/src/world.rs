use api::{
    block::BlockPosition,
    chunk::Chunk,
    world::{ChunkGenerator, ChunkLocation},
};

pub struct StandardWorldGenerator;

impl ChunkGenerator for StandardWorldGenerator {
    fn generate_chunk(&self, _location: ChunkLocation) -> Chunk {
        let mut chunk = Chunk::new();
        for x in 0..16 {
            for y in 0..16 {
                chunk.add_block(BlockPosition(x, y, 10), "stone".to_string());
            }
        }
        chunk
    }
}

pub struct SinusoidWorldGenerator;

impl ChunkGenerator for SinusoidWorldGenerator {
    fn generate_chunk(&self, location: ChunkLocation) -> Chunk {
        let mut chunk = Chunk::new();
        if location.2 != -1 {
            return chunk;
        }
        for x in 0..16 as u8{
            for y in 0..16 as u8 {
                let height = ((location.0 + (x as i32)) as f32).sin() * 5.0
                    + ((location.1 + (y as i32)) as f32).sin() * 5.0
                    + 5.0;
                for z in 0..height as u8 {
                    chunk.add_block(BlockPosition(x, y, z), "stone".to_string());
                }
            }
        }
        chunk
    }
}
