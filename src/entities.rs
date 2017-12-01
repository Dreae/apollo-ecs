use super::Entity;
use std::any::{Any, TypeId};
use std::sync::{Arc, Mutex};

pub type Components = Arc<Mutex<Vec<Component>>>;
pub type Component = (TypeId, *mut Any);

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

        components.push((TypeId::of::<T>(), Box::into_raw(Box::new(component))));
    }

    pub fn has<T>(&self) -> bool where T: Any {
        let ty = TypeId::of::<T>();
        let components = self.components.lock().unwrap();
        for &(comp_ty, _) in components.iter() {
            if comp_ty == ty {
                return true;
            }
        }

        false
    }

    pub fn get<T>(&self) -> Option<&T> where T: Any {
        let ty = TypeId::of::<T>();
        let components = self.components.lock().unwrap();
        for &(comp_ty, ptr) in components.iter() {
            if comp_ty == ty {
                unsafe {
                    return (*ptr).downcast_ref();
                }
            }
        }

        None
    }
}

impl Into<Entity> for EntityEditor {
    fn into(self) -> Entity {
        self.ent
    }
}