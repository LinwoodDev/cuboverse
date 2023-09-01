use std::{sync::MutexGuard, time::{Duration, SystemTime}};

use super::*;

#[derive(Clone)]
pub struct WorldManager {
    pub(crate) world: RustOpaque<Mutex<World>>,
    pub(crate) messenger: RustOpaque<Mutex<WorldMessenger>>,
    pub(crate) loaded_chunks: RustOpaque<Mutex<Vec<ChunkLocation>>>,
    pub(crate) chunk_generator : RustOpaque<Box<dyn ChunkGenerator>>,
    pub(crate) player: RustOpaque<Mutex<Player>>,
    pub(crate) update_thread: RustOpaque<Mutex<WorldTicker>>,
}
const UPDATE_INTERVAL: Duration = Duration::from_millis(1000 / 20);

pub struct WorldTicker(pub Option<JoinHandle<()>>);

impl Drop for WorldTicker {
    fn drop(&mut self) {
        if let Some(h) = self.0.take() {
            h.join().unwrap();
        }
    }
}

impl WorldManager {
    pub fn get_messenger(&self) -> MutexGuard<'_, WorldMessenger> {
        self.messenger.lock().unwrap()
    }
    pub fn get_world(&self) -> MutexGuard<'_, World> {
        self.world.lock().unwrap()
    }
    pub fn get_chunk_generator(&self) -> &dyn ChunkGenerator {
        self.chunk_generator
    }
    pub fn get_chunk(location : ChunkLocation) -> Chunk {
        self.get_world().get_chunk(location, &self.get_chunk_generator())
    }
    
    pub(crate) fn start_update_loop(&self) {
        self.update_thread.lock().unwrap().0 = Some({
            let world = self.world.clone();
            let player = self.player.clone();
            let messenger = self.messenger.clone();
            let loaded_chunks = self.loaded_chunks.clone();
            thread::spawn(move || {
                let mut last_update = SystemTime::now();
                while messenger.lock().unwrap().0.is_some() {
                    {
                        let mut world = world.lock().unwrap();
                        world.tick();
                        let mut player = player.lock().unwrap();
                        let old_chunk = player.position.0;
                        let result = player.tick_player(&mut world);
                        let messenger = messenger.lock().unwrap();
                        if result.teleported {
                            messenger.send_player_teleported(&player);
                            if old_chunk != player.position.0 {
                                let mut chunks = loaded_chunks.lock().unwrap();
                                Self::run_test_chunks(
                                    &player.position.0,
                                    &mut chunks,
                                    &messenger,
                                    &mut world,
                                );
                            }
                        }
                    }
                    let now = SystemTime::now();
                    let elapsed = now.duration_since(last_update).unwrap();
                    if elapsed < UPDATE_INTERVAL {
                        thread::sleep(UPDATE_INTERVAL - elapsed);
                    }
                    last_update = now;
                }
            })
        });
        self.init_world();
    }

    pub(crate) fn init_world(&self) {
        self.get_messenger()
            .send_player_teleported(&self.player.lock().unwrap());
        self.test_chunks();
    }
}
