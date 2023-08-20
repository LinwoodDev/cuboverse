use flutter_rust_bridge::RustOpaque;
pub use api::world::World;

pub mod manager;

#[derive(Debug)]
pub struct WorldManager {
    pub(crate) world: RustOpaque<World>,
}

pub fn create_world_manager() -> WorldManager {
    WorldManager {
        world: RustOpaque::new(World {
            ..Default::default()
        }),
    }
}

pub fn what_is_the_answer() -> i32 {
    21
}

pub fn world_manager_info(manager: &WorldManager) -> String {
    format!("WorldManager: {:?}", manager)
}

impl WorldManager {
    pub fn hello() {
        println!("Hello from Rust!");
    }

    pub fn concatenate_static(a: String, b: String) -> String {
        format!("{a}{b}")
    }
}

