use crate::{
    entity::*,
    physics::*,
    world::{ChunkLocation, World},
};

pub struct Player {
    pub position: GlobalEntityPosition,
    pub name: String,
    pub velocity: Velocity,
}

impl RigidBody for Player {
    fn get_velocity(&self) -> Velocity {
        self.velocity
    }

    fn set_velocity(&mut self, velocity: Velocity) {
        self.velocity = velocity;
    }

    fn get_position(&self) -> EntityPosition {
        self.position.1
    }

    fn set_global_position(
        &mut self,
        position: GlobalEntityPosition,
        _old: ChunkLocation,
        _world: &mut World,
    ) {
        self.position = position;
    }
}

impl Player {
    pub fn tick_player(&mut self, world: &mut World) -> TickResult {
        self.tick(world, self.position.0)
    }

    pub fn move_player(
        &mut self,
        x: Option<f64>,
        y: Option<f64>,
        z: Option<f64>,
        relative: Option<bool>,
        teleport: Option<bool>,
    ) {
        if teleport.unwrap_or(true) {
            let global_pos = self.position.global_position();
            self.position = if relative.unwrap_or(true) {
                self.position
                    .move_position(x.unwrap_or(0.0), y.unwrap_or(0.0), z.unwrap_or(0.0))
            } else {
                GlobalEntityPosition::new(
                    x.unwrap_or(global_pos.0),
                    y.unwrap_or(global_pos.1),
                    z.unwrap_or(global_pos.2),
                )
            };
        } else {
            self.velocity = if relative.unwrap_or(true) {
                self.velocity + Velocity(x.unwrap_or(0.0), y.unwrap_or(0.0), z.unwrap_or(0.0))
            } else {
                Velocity(
                    x.unwrap_or(self.velocity.0),
                    y.unwrap_or(self.velocity.1),
                    z.unwrap_or(self.velocity.2),
                )
            };
        }
    }
}
