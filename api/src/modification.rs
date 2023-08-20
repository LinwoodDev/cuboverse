use crate::world::World;

#[derive(Debug, Clone)]
pub struct ModificationInfo {
    pub name: String,
    pub description: String,
    pub author: String,
    pub version: String,
}

trait Moditification {
    fn info(&self) -> ModificationInfo;
    fn load_world(&self, world: &World);
    fn unload_world(&self, world: &World);
}
