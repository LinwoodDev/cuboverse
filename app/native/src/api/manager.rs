use flutter_rust_bridge::RustOpaque;
use api::world::{ChunkLocation, ChunkPosition, Entity};
pub use api::world::World;
use crate::api::WorldManager;

impl WorldManager {
    pub fn new() -> Self {
        Self {
            world: RustOpaque::new(World {
                ..Default::default()
            }),
        }
    }

    pub fn add_entity(&mut self, entity: String) -> Result<(), RustOpaque<World>> {
        let mut world = self.world.clone().try_unwrap()?;
        world.entities.insert(ChunkLocation(0, 0, 0), vec![Entity {
            position: ChunkPosition {
                x: 0,
                y: 0,
                z: 0,
            },
            name: entity,
            health: 100,
            max_health: 100,
        }]);
        Ok(())
    }
    pub fn entities(&self) -> Vec<Entity> {
        // world is an Option<Arc<World>> which has a method try_unwrap
        self.world.clone().try_unwrap().unwrap().entities.get(&ChunkLocation(0, 0, 0)).unwrap().clone()
    }
}
