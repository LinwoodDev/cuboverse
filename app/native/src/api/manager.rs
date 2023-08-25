use std::{sync::MutexGuard, time::Duration};

use super::*;

#[derive(Clone)]
pub struct WorldManager {
    pub(crate) world: RustOpaque<Mutex<World>>,
    pub(crate) messenger: RustOpaque<Mutex<WorldMessenger>>,
    pub(crate) loaded_chunks: RustOpaque<Mutex<Vec<ChunkLocation>>>,
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
    pub(crate) fn start_update_loop(&self) {
        let world = self.world.clone();
        let player = self.player.clone();
        let messenger = self.messenger.clone();
        let handle = thread::spawn(move || {
            while messenger.lock().unwrap().0.is_some() {
                {
                    world.lock().unwrap().tick();
                    let mut player = player.lock().unwrap();
                    let old_chunk = player.position.0;
                    let result = player.tick_player(&mut *world.lock().unwrap());
                    let messenger = messenger.lock().unwrap();
                    if result.teleported {
                        messenger.send_player_teleported(&player);
                        if old_chunk != player.position.0 {
                            self.test_chunks();
                        }
                    }
                }
                thread::sleep(UPDATE_INTERVAL);
            }
        });
        self.init_world();
        self.update_thread.lock().unwrap().0 = Some(handle);
    }

    pub(crate) fn init_world(&self) {
        self.get_messenger()
            .send_player_teleported(&self.player.lock().unwrap());
        self.test_chunks();
    }
}
