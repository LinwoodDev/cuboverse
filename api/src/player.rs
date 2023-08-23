use crate::{physics::*, entity::*, world::{ChunkLocation, World}};

pub struct Player {
    pub position: GlobalEntityPosition,
    pub name: String,
    pub velocity : Veloctiy,
}

impl RigidBody for Player {
    fn get_velocity(&self) -> Veloctiy {
        self.velocity
    }

    fn set_velocity(&mut self, velocity: Veloctiy) {
        self.velocity = velocity;
    }

    fn get_position(&self) -> EntityPosition {
        self.position.1
    }

    fn set_global_position(&mut self, position: GlobalEntityPosition, _old : ChunkLocation, _world : &mut World) {
        self.position = position;
    }
}

impl Player {
    pub fn tick_player(&mut self, world : &mut World) -> TickResult {
        self.tick(world, self.position.0.clone())
    }
}
