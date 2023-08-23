use crate::{entity::*, world::{World, ChunkLocation}};

pub const MAX_VELOCITY: f32 = 1.0;
pub const GRAVITY: f32 = 0.1;
pub const FRICITON: f32 = 0.1;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct TickResult {
    pub teleported : bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Veloctiy(pub f32, pub f32, pub f32);

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

pub trait RigidBody {
    fn get_velocity(&self) -> Veloctiy;
    fn set_velocity(&mut self, velocity: Veloctiy);
    fn get_position(&self) -> EntityPosition;
    fn get_global_position(&self, chunk_location : ChunkLocation) -> GlobalEntityPosition {
        return GlobalEntityPosition(chunk_location, self.get_position());
    }
    fn set_global_position(&mut self, position: GlobalEntityPosition, old : ChunkLocation, world : &mut World);

    fn tick(&mut self, world : &mut World, chunk_location : ChunkLocation) -> TickResult {
        let old = self.get_velocity();
        let mut velocity = old.clone();
        velocity.2 -= GRAVITY;
        if velocity.2 < -MAX_VELOCITY {
            velocity.2 = -MAX_VELOCITY;
        }

        // Apply friction
        if velocity.0.abs() > FRICITON {
            velocity.0 -= FRICITON * velocity.0.signum();
        } else {
            velocity.0 = 0.0;
        }
        if velocity.1.abs() > FRICITON {
            velocity.1 -= FRICITON * velocity.1.signum();
        } else {
            velocity.1 = 0.0;
        }
        let global_pos = self.get_global_position(chunk_location);
        let block_position = global_pos.get_block_position();
        let chunk = world.get_chunk(block_position.0);
        if chunk.has_block(block_position.1) {
            velocity.2 = 0.0;
        }
        self.set_velocity(velocity);
        let new_pos = global_pos.move_position(velocity.0 as f64, velocity.1 as f64, velocity.2 as f64);
        println!("Moving entity with {:?} to {:?}, old: {:?}", velocity, global_pos, old);
        self.set_global_position(new_pos, chunk_location, world);
        return TickResult { teleported: new_pos != global_pos };
    }
}
