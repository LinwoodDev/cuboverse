pub mod manager;
pub mod message;

pub use api::chunk::*;
pub use api::world::*;
pub use std::sync::Mutex;

use flutter_rust_bridge::{RustOpaque, StreamSink};

pub use self::manager::*;
pub use self::message::*;

pub fn create_world_manager() -> WorldManager {
    WorldManager {
        world: RustOpaque::new(Mutex::new(World {
            ..Default::default()
        })),
        sink: RustOpaque::new(Mutex::new(None)),
        player: RustOpaque::new(Mutex::new(Player {
            position: GlobalPosition(ChunkLocation(0, 0, 0), ChunkPosition(0, 0, 0)),
            name: "Player".to_string(),
        })),
    }
}

pub fn what_is_the_answer() -> i32 {
    42
}

impl WorldManager {
    pub fn add_block(&self, position: GlobalPosition, block: String) {
        let mut world = self.world.lock().unwrap();
        world.get_chunk(position.0).add_block(position.1, block);
    }
    pub fn remove_block(&self, position: GlobalPosition) {
        let mut world = self.world.lock().unwrap();
        world
            .get_chunk(position.0.clone())
            .remove_block(&position.1);
    }
    pub fn add_entity(&self, entity: String) {
        let mut world = self.world.lock().unwrap();
        let key = ChunkLocation(0, 0, 0);
        let new_value = Entity {
            position: ChunkPosition(0, 0, 0),
            name: entity,
            health: 100,
            max_health: 100,
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
        *self.sink.lock().unwrap() = Some(s);
    }
    pub fn player_position(&self) -> GlobalPosition {
        let player = self.player.lock().unwrap();
        player.position.clone()
    }
    pub fn send_message(&self, message: NativeMessage) {
        let sink = self.sink.lock().unwrap();
        if let Some(s) = sink.as_ref() {
            s.add(message);
        }
    }
    pub fn move_player(&self, x: i64, y: i64, z: i64) {
        let mut player = self.player.lock().unwrap();
        player.position = player.position.move_position(x, y, z);
        let (x, y, z) = player.position.global_position();
        self.send_message(NativeMessage::PlayerTeleported { x, y, z });
    }
}
