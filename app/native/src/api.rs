pub mod manager;
pub mod message;

pub use api::{chunk::*, world::*, physics::*,player::*, entity::*, block::*};
pub use std::sync::Mutex;
pub use std::thread;
pub use std::thread::JoinHandle;

use flutter_rust_bridge::{RustOpaque, StreamSink};

pub use self::manager::*;
pub use self::message::*;

pub fn create_world_manager() -> WorldManager {
    WorldManager {
        world: RustOpaque::new(Mutex::new(World {
            ..Default::default()
        })),
        messenger: RustOpaque::new(Mutex::new(WorldMessenger(None))),
        player: RustOpaque::new(Mutex::new(Player {
            position: GlobalEntityPosition(ChunkLocation(0, 0, 0), EntityPosition(0.0, 0.0, 0.0)),
            name: "Player".to_string(),
            velocity: Veloctiy(0.0, 0.0, 0.0),
        })),
        update_thread: RustOpaque::new(Mutex::new(WorldTicker(None))),
    }
}

pub fn what_is_the_answer() -> i32 {
    42
}

impl WorldManager {
    pub fn add_block(&self, position: GlobalBlockPosition, block: String) {
        let mut world = self.world.lock().unwrap();
        let block = world.get_chunk(position.0).add_block(position.1, block);
        if let Some(block) = block {
            self.get_messenger().send_add_block(position, &block);
        }
    }
    pub fn remove_block(&self, position: GlobalBlockPosition) {
        let mut world = self.world.lock().unwrap();
        world
            .get_chunk(position.0.clone())
            .remove_block(&position.1);
    }
    pub fn add_entity(&self, entity: String) {
        let mut world = self.world.lock().unwrap();
        let key = ChunkLocation(0, 0, 0);
        let new_value = Entity {
            position: EntityPosition(0.0, 0.0, 0.0),
            name: entity,
            health: 100,
            max_health: 100,
            velocity: Veloctiy(0.0, 0.0, 0.0),
        };
        let entities = &mut world.entities;
        let currents = entities.get_mut(&key);
        if let Some(k) = currents {
            k.push(new_value);
        } else {
            entities.insert(key, vec![new_value]);
        }
    }
    pub fn entities(&self) -> usize {
        let world = self.world.lock().unwrap();
        world.entities.values().flatten().count()
    }
    pub fn create_message_stream(&self, s: StreamSink<NativeMessage>) {
        self.get_messenger().0 = Some(s);
        self.start_update_loop();
    }
    pub fn close(&self) {
        self.get_messenger().0 = None;
    }
    pub fn player_position(&self) -> GlobalEntityPosition {
        let player = self.player.lock().unwrap();
        player.position.clone()
    }
    pub fn move_player(&self, x: f64, y: f64, z: f64) {
        let mut player = self.player.lock().unwrap();
        player.position = player.position.move_position(x, y, z);
        let (x, y, z) = player.position.global_position();
        self.get_messenger().send_message(NativeMessage::PlayerTeleported { x, y, z });
    }
}
