pub mod message;

pub use std::sync::Mutex;
pub use api::chunk::*;
pub use api::world::*;

use flutter_rust_bridge::{RustOpaque, StreamSink};

pub use self::message::*;

pub struct WorldManager {
    pub(crate) world: RustOpaque<Mutex<World>>,
    pub(crate) sink : RustOpaque<Mutex<Option<StreamSink<NativeMessage>>>>,
}

pub fn create_world_manager() -> WorldManager {
    WorldManager {
        world: RustOpaque::new(Mutex::new(World {
            ..Default::default()
        })),
        sink: RustOpaque::new(Mutex::new(None)),
    }
}

pub fn what_is_the_answer() -> i32 {
    42
}

impl WorldManager {
    pub fn add_entity(&self, entity: String) {
        let mut world = self.world.lock().unwrap();
        let key = ChunkLocation(0, 0, 0);
        let new_value = Entity {
            position: ChunkPosition(0,0,0),
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
}
