use super::{Components, Entity};

use std::any::{Any, TypeId};

pub struct EntityEditor {
    ent: Entity,
    components: Components
}

impl EntityEditor {
    pub fn new(ent: Entity, components: Components) -> EntityEditor {
        EntityEditor {
            ent,
            components
        }
    }

    pub fn add<T>(&self, component: T) where T: Any {
        let mut components = self.components.lock().unwrap();

        components.push((TypeId::of::<T>(), Box::new(component)));
    }
}

impl Into<Entity> for EntityEditor {
    fn into(self) -> Entity {
        self.ent
    }
}