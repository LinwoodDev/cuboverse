use std::{time::Duration, sync::MutexGuard};

use super::*;

#[derive(Clone)]
pub struct WorldManager {
    pub(crate) world: RustOpaque<Mutex<World>>,
    pub(crate) messenger: RustOpaque<Mutex<WorldMessenger>>,
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
    pub(crate) fn start_update_loop(&self) {
        let world = self.world.clone();
        let player = self.player.clone();
        let messenger = self.messenger.clone();
        let handle = thread::spawn(move || {
            while messenger.lock().unwrap().0.is_some() {
                world.lock().unwrap().tick();
                let mut player = player.lock().unwrap();
                let result = player.tick_player(&mut *world.lock().unwrap());
                let messenger = messenger.lock().unwrap();
                if result.teleported {
                    messenger.send_player_teleported(&player);
                }
                thread::sleep(UPDATE_INTERVAL);
            }
        });
        self.update_thread.lock().unwrap().0 = Some(handle);
    }
}
