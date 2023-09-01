use std::ops::{Add, Sub};

use crate::{
    entity::*,
    world::{ChunkLocation, World},
};

pub const MAX_VELOCITY: f64 = 0.5;
pub const GRAVITY: f64 = 0.01;
pub const FRICITON: f64 = 1.0;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct TickResult {
    pub teleported: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Velocity(pub f64, pub f64, pub f64);

impl Add<Velocity> for Velocity {
    type Output = Velocity;

    fn add(self, rhs: Velocity) -> Self::Output {
        Velocity(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub<Velocity> for Velocity {
    type Output = Velocity;

    fn sub(self, rhs: Velocity) -> Self::Output {
        Velocity(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

pub trait RigidBody {
    fn get_velocity(&self) -> Velocity;
    fn set_velocity(&mut self, velocity: Velocity);
    fn get_position(&self) -> EntityPosition;
    fn get_global_position(&self, chunk_location: ChunkLocation) -> GlobalEntityPosition {
        GlobalEntityPosition(chunk_location, self.get_position())
    }
    fn set_global_position(
        &mut self,
        position: GlobalEntityPosition,
        old: ChunkLocation,
        world: &mut World,
    );

    fn tick(&mut self, world: &mut World, chunk_location: ChunkLocation) -> TickResult {
        let old = self.get_velocity();
        let mut velocity = old;
        velocity.2 -= GRAVITY;
        if velocity.2 < -MAX_VELOCITY {
            velocity.2 = -MAX_VELOCITY;
        }

        let global_pos = self.get_global_position(chunk_location);
        let on_ground = world.has_block(global_pos.get_offset_block_position(0.0, 0.0, -1.0));
        if (on_ground
            && velocity.2 < 0.0)
            || (world.has_block(global_pos.get_offset_block_position(0.0, 0.0, 1.0))
                && velocity.2 > 0.0)
        {
            velocity.2 = 0.0;
        }
        if (world.has_block(global_pos.get_offset_block_position(0.0, -1.0, 0.0))
            && velocity.1 < 0.0)
            || (world.has_block(global_pos.get_offset_block_position(0.0, 1.0, 0.0))
                && velocity.1 > 0.0)
        {
            velocity.1 = 0.0;
        }
        if (world.has_block(global_pos.get_offset_block_position(-1.0, 0.0, 0.0))
            && velocity.0 < 0.0)
            || (world.has_block(global_pos.get_offset_block_position(1.0, 0.0, 0.0))
                && velocity.0 > 0.0)
        {
            velocity.0 = 0.0;
        }
        let new_pos = global_pos.move_position(velocity.0, velocity.1, velocity.2);
        // Apply friction
        if velocity.0.abs() > FRICITON {
            velocity.0 -= FRICITON * velocity.0.signum();
        } else {
            velocity.0 = 0.0;
        }
        if velocity.1.abs() > FRICITON  {
            velocity.1 -= FRICITON * velocity.1.signum();
        } else {
            velocity.1 = 0.0;
        }
        self.set_velocity(velocity);
        self.set_global_position(new_pos, chunk_location, world);
        TickResult {
            teleported: new_pos != global_pos,
        }
    }
}
