use crate::{world::{World, ChunkLocation, CHUNK_SIZE}, block::{BlockPosition, GlobalBlockPosition}, physics::{Velocity, RigidBody}};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EntityPosition(pub f32, pub f32, pub f32);

impl EntityPosition {
    pub fn is_valid(&self) -> bool {
        self.0 >= 0.0
            && self.1 >= 0.0
            && self.2 >= 0.0
            && self.0 < CHUNK_SIZE as f32
            && self.1 < CHUNK_SIZE as f32
            && self.2 < CHUNK_SIZE as f32
    }

    pub fn get_block_position(&self) -> BlockPosition {
        BlockPosition(self.0 as i8, self.1 as i8, self.2 as i8)
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GlobalEntityPosition(pub ChunkLocation, pub EntityPosition);

impl GlobalEntityPosition {
    pub fn new(x : f64, y : f64, z: f64) -> Self {
        let chunk_size = CHUNK_SIZE as f64;
        let location = ChunkLocation(
            (x / chunk_size) as i32 + if x < 0.0 { -1 } else { 0 },
            (y / chunk_size) as i32 + if y < 0.0 { -1 } else { 0 },
            (z / chunk_size) as i32 + if z < 0.0 { -1 } else { 0 },
        );
        let position = EntityPosition(
            ((x + if x < 0.0 { chunk_size } else { 0.0 }) % chunk_size) as f32,
            ((y + if y < 0.0 { chunk_size } else { 0.0 }) % chunk_size) as f32,
            ((z + if z < 0.0 { chunk_size } else { 0.0 }) % chunk_size) as f32,
        );
        return GlobalEntityPosition(location, position);
    }

    pub fn global_position(&self) -> (f64, f64, f64) {
        (
            self.0.0 as f64 * CHUNK_SIZE as f64 + self.1.0 as f64,
            self.0.1 as f64 * CHUNK_SIZE as f64 + self.1.1 as f64,
            self.0.2 as f64 * CHUNK_SIZE as f64 + self.1.2 as f64,
        )
    }

    pub fn get_block_position(&self) -> GlobalBlockPosition {
        GlobalBlockPosition(self.0, self.1.get_block_position())
    }

    pub fn get_offset_block_position(&self, x:f64, y:f64, z:f64) -> GlobalBlockPosition {
        self.move_position(x,y,z).get_block_position()
    }

    pub fn move_position(&self, x : f64, y : f64, z: f64) -> GlobalEntityPosition {
        let current = self.global_position();
        GlobalEntityPosition::new(current.0 + x, current.1 + y, current.2 + z)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Entity {
    pub position: EntityPosition,
    pub velocity : Velocity,
    pub name: String,
    pub health: i32,
    pub max_health: i32,
}

impl RigidBody for Entity {
    fn get_velocity(&self) -> Velocity {
        self.velocity 
    }

    fn set_velocity(&mut self, velocity: Velocity) {
        self.velocity = velocity;
    }

    fn get_position(&self) -> EntityPosition {
        self.position
    }

    fn set_global_position(&mut self, position: GlobalEntityPosition, old : ChunkLocation, world : &mut World) {
        self.position = position.1;
        world.move_entity(self, old, position.0);
    }
}
