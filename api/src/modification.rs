use std::collections::HashMap;
use crate::world::World;

#[derive(Debug, Clone)]
pub struct ModificationInfo {
    pub name: String,
    pub description: String,
    pub author: String,
    pub version: String,
}

pub trait Moditification {
    fn load_world(&self, world: &World);
    fn unload_world(&self, world: &World);
}

pub struct ModificationState {
    pub info: ModificationInfo,
    pub modification: Box<dyn Moditification>,
    pub blocks : HashMap<String, BlockType>,
    pub items : HashMap<String, ItemType>,
    pub localization : HashMap<String, LocalizationFile>,
}

#[derive(Debug, Clone)]
pub struct BlockType {
    pub texture: String,
}

#[derive(Debug, Clone)]
pub struct ItemType {
    pub texture: String,
}

#[derive(Debug, Clone)]
pub struct LocalizationFile(pub HashMap<String, String>);
