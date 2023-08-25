use super::*;

const RENDER_DISTANCE: u8 = 4;

impl WorldManager {
    pub fn test_chunks(&self) {
        self.test_chunks_unloading();
        self.test_chunks_loading();
    }
    fn test_chunks_unloading(&self) {
        let player_location = &self.player_position().0;
        self.loaded_chunks.lock().unwrap().retain(|e| {
            let keep = e.distance(player_location) <= RENDER_DISTANCE.into();
            if !keep {
                self.get_messenger().send_remove_chunk(e.clone());
            }
            keep
        });
    }

    fn test_chunks_loading(&self) {
        let player_location = &self.player_position().0;
        let distance = RENDER_DISTANCE as i32;
        for x in -distance..=distance {
            for y in -distance..=distance {
                for z in -distance..=distance {
                    let location = ChunkLocation(
                        x + player_location.0,
                        y + player_location.1,
                        z + player_location.2,
                    );
                    let mut chunks = self.loaded_chunks.lock().unwrap();
                    println!("Load {:?}? {:?}", &location, chunks.contains(&location));
                    if !chunks.contains(&location) {
                        chunks.push(location);
                        self.get_messenger()
                            .send_add_chunk(location, self.get_world().get_chunk(location));
                    }
                }
            }
        }
    }
}
