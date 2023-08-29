use super::*;

struct ModificationManager {
    pub modifications: Vec<ModificationState>,
}

impl ModificationManager {
    pub fn load_modification(&self) {

    }
    pub fn init_world(&self, world: &World) {
        for modification in &self.modifications {
            modification.load_world(world);
        }
    }

    pub fn unload_world(&self, world: &World) {
        for modification in &self.modifications {
            modification.unload_world(world);
        }
    }
}
