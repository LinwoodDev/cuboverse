pub mod loader;
pub mod manager;
pub mod message;
pub mod modification;

pub use api::{block::*, chunk::*, entity::*, physics::*, player::*, world::*, modification::*};
pub use std::sync::Mutex;
pub use std::thread;
pub use std::thread::JoinHandle;

use flutter_rust_bridge::{RustOpaque, StreamSink, SyncReturn};
use standard::world::SinusoidWorldGenerator;

pub use self::loader::*;
pub use self::manager::*;
pub use self::message::*;

pub fn create_world_manager() -> WorldManager {
    let player = Player {
        position: GlobalEntityPosition(ChunkLocation(0, 0, 0), EntityPosition(0.0, 0.0, 0.0)),
        name: "Player".to_string(),
        velocity: Velocity(0.0, 0.0, 0.0),
    };
    WorldManager {
        loaded_chunks: RustOpaque::new(Mutex::new(Vec::new())),
        world: RustOpaque::new(Mutex::new(World::new())),
        messenger: RustOpaque::new(Mutex::new(WorldMessenger(None))),
        player: RustOpaque::new(Mutex::new(player)),
        update_thread: RustOpaque::new(Mutex::new(WorldTicker(None))),
        chunK_generator: RustOpaque::new(Box::new(SinusoidWorldGenerator {})),
    }
}

pub fn what_is_the_answer() -> i32 {
    42
}

impl WorldManager {
    pub fn add_block(&self, position: GlobalBlockPosition, block: String) {
        let block = self
            .get_world()
            .get_chunk(position.0)
            .add_block(position.1, block);
        if let Some(block) = block {
            self.get_messenger().send_add_block(position, &block);
        }
    }
    pub fn remove_block(&self, position: GlobalBlockPosition) {
        self.get_world()
            .get_chunk(position.0.clone())
            .remove_block(&position.1);
    }
    pub fn add_entity(&self, entity: String) {
        let key = ChunkLocation(0, 0, 0);
        let new_value = Entity {
            position: EntityPosition(0.0, 0.0, 0.0),
            name: entity,
            health: 100,
            max_health: 100,
            velocity: Velocity(0.0, 0.0, 0.0),
        };
        let entities = &mut self.get_world().entities;
        let currents = entities.get_mut(&key);
        if let Some(k) = currents {
            k.push(new_value);
        } else {
            entities.insert(key, vec![new_value]);
        }
    }
    pub fn entities(&self) -> usize {
        self.get_world().entities.values().flatten().count()
    }
    pub fn create_message_stream(&self, s: StreamSink<NativeMessage>) {
        self.get_messenger().0 = Some(s);
        self.start_update_loop();
    }
    pub fn close(&self) -> SyncReturn<()> {
        if let Some(messenger) = &self.get_messenger().0 {
            messenger.close();
        }
        self.get_messenger().0 = None;
        SyncReturn(())
    }
    pub fn player_position(&self) -> GlobalEntityPosition {
        let player = self.player.lock().unwrap();
        player.position.clone()
    }
    pub fn move_player(
        &self,
        x: Option<f64>,
        y: Option<f64>,
        z: Option<f64>,
        relative: Option<bool>,
        teleport: Option<bool>,
    ) {
        let should_test_chunks = {
            let mut player = self.player.lock().unwrap();
            let old_chunk = player.position.0;
            player.move_player(x, y, z, relative, teleport);
            self.get_messenger().send_player_teleported(&player);
            old_chunk != player.position.0
        };
        if should_test_chunks {
            self.test_chunks();
        }
    }
    pub fn player_on_ground(&self) -> bool {
        let player = self.player.lock().unwrap();
        self.get_world()
            .has_block(player.position.get_offset_block_position(0.0, 0.0, -1.0))
    }
}
