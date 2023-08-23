use api::{
    block::BlockPosition,
    world::{ChunkLocation, World},
};

pub fn create_standard_world() -> World {
    let mut world = World::new();
    for x in -1..=1 {
        for y in -1..=1 {
            let chunk = world.get_chunk(ChunkLocation(x, y, -1));
            for x in 0..16 {
                for y in 0..16 {
                    chunk.add_block(BlockPosition(x, y, 10), "stone".to_string());
                }
            }
        }
    }

    world
}
