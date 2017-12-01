use std::sync::{Mutex, RwLock, Arc};
use std::collections::VecDeque;

use super::{Entity, Components};
use super::entities::EntityEditor;

pub struct World {
    entities: RwLock<Vec<Components>>,
    free_ents: Mutex<VecDeque<usize>>
}

impl World {
    pub fn new() -> World {
        World::with_capacity(524288)
    }

    pub fn with_capacity(capacity: usize) -> World {
        World {
            entities: RwLock::new(Vec::with_capacity(capacity)),
            free_ents: Mutex::new(VecDeque::with_capacity(capacity / 3)),
        }
    }

    pub fn create_entity(&self) -> Entity {
        let mut free_ents = self.free_ents.lock().unwrap();
        if free_ents.is_empty() {
            let mut entities = self.entities.write().unwrap();
            entities.push(Arc::new(Mutex::new(Vec::with_capacity(12))));

            entities.len() - 1
        } else {
            let ent = free_ents.pop_front().unwrap();
            let ents = self.entities.read().unwrap();

            ents.get(ent).unwrap().lock().unwrap().truncate(0);

            ent
        }
    }

    pub fn edit(&self, ent: Entity) -> Option<EntityEditor> {
        let entities = self.entities.read().unwrap();
        if let Some(components) = entities.get(ent) {
            Some(EntityEditor::new(ent, components.clone()))
        } else {
            None
        }
    }
}