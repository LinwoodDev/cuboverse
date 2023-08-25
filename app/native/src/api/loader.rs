use super::*;

const RENDER_DISTANCE: u8 = 2;

impl WorldManager {
    pub fn test_chunks(&self) {
        let mut loaded_chunks = self.loaded_chunks.lock().unwrap();
        Self::run_test_chunks(
            &self.player.lock().unwrap().position.0,
            &mut loaded_chunks,
            &self.get_messenger(),
            &mut self.get_world(),
        );
    }
    pub(crate) fn run_test_chunks(
        player_position: &ChunkLocation,
        loaded_chunks: &mut Vec<ChunkLocation>,
        messenger: &WorldMessenger,
        world: &mut World,
    ) {
        Self::test_chunks_unloading(player_position, loaded_chunks, messenger);
        Self::test_chunks_loading(player_position, loaded_chunks, messenger, world);
    }
    pub(crate) fn test_chunks_unloading(
        player_location: &ChunkLocation,
        loaded_chunks: &mut Vec<ChunkLocation>,
        messenger: &WorldMessenger,
    ) {
        loaded_chunks.retain(|e| {
            let keep = e.distance(player_location) <= RENDER_DISTANCE.into();
            if !keep {
                messenger.send_remove_chunk(e.clone());
            }
            keep
        });
    }

    pub(crate) fn test_chunks_loading(
        player_location: &ChunkLocation,
        loaded_chunks: &mut Vec<ChunkLocation>,
        messenger: &WorldMessenger,
        world: &mut World,
    ) {
        let distance = RENDER_DISTANCE as i32;
        for x in -distance..=distance {
            for y in -distance..=distance {
                for z in -distance..=distance {
                    let location = ChunkLocation(
                        x + player_location.0,
                        y + player_location.1,
                        z + player_location.2,
                    );
                    if !loaded_chunks.contains(&location) {
                        loaded_chunks.push(location);
                        messenger.send_add_chunk(location, &world.get_chunk(location));
                    }
                }
            }
        }
    }
}
