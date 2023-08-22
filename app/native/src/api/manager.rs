use super::*;

pub struct WorldManager {
    pub(crate) world: RustOpaque<Mutex<World>>,
    pub(crate) sink : RustOpaque<Mutex<Option<StreamSink<NativeMessage>>>>,
    pub(crate) player : RustOpaque<Mutex<Player>>,
}

pub struct Player {
    pub position: GlobalPosition,
    pub name: String,
}

