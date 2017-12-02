use super::Entity;
use super::entities::{EntityEditor, Components};
use super::query::{Query, QueryRunner};
use super::systems::IterativeSystem;
use std::ops::DerefMut;

pub struct World {
    pub(crate) entities: Vec<Components>,
    iterative_systems: Vec<(Box<IterativeSystem>, Query)>
}

impl World {
    pub fn new() -> World {
        World::with_capacity(524288)
    }

    pub fn with_capacity(capacity: usize) -> World {
        World {
            entities: Vec::with_capacity(capacity),
            iterative_systems: Vec::new()
        }
    }

    pub fn register_iterative_system<T>(&mut self, system: T) where T: IterativeSystem + 'static {
        self.iterative_systems.push((Box::new(system), T::get_query()));
    } 

    pub fn create_entity(&mut self) -> Entity {
        self.entities.push(Vec::with_capacity(12));

        self.entities.len() - 1
    }

    pub fn drop_entity(&mut self, ent: Entity) {
        if ent < self.entities.len() {
            let components = self.entities.remove(ent);
            for comp in components {
                unsafe {
                    // Drop component memory
                    Box::from_raw(comp.1);
                }
            }
        }
    }

    pub fn edit(&mut self, ent: Entity) -> Option<EntityEditor> {
        if let Some(components) = self.entities.get_mut(ent) {
            Some(EntityEditor::new(ent, components))
        } else {
            None
        }
    }

    pub fn filter_entities<'a, 'q>(&'a mut self, query: &'q Query) -> QueryRunner<'q> {
        QueryRunner::new(self.entities.as_ptr(), self.entities.len(), query)
    }

    pub fn process(&mut self) {
        for sys in self.iterative_systems.iter_mut() {
            for ent in QueryRunner::new(self.entities.as_ptr(), self.entities.len(), &sys.1) {
                sys.0.deref_mut().process(EntityEditor::new(ent, self.entities.get_mut(ent).unwrap()));
            }
        }
    }
}