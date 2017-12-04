use super::Entity;
use super::entities::{EntityEditor, Components};
use super::query::{Query, QueryRunner, Condition};
use super::systems::IterativeSystem;

use std::cell::RefCell;
use std::collections::VecDeque;

pub struct World {
    pub(crate) entities: Vec<(bool, RefCell<Components>)>,
    iterative_systems: Vec<(RefCell<Box<IterativeSystem>>, Query)>,
    free_ents: VecDeque<Entity>
}

impl World {
    /// Create a new ECS world with a default capacity for entities of 131072
    pub fn new() -> World {
        World::with_capacity(131072)
    }

    /// Create a new world with custom initial capacity specified
    pub fn with_capacity(capacity: usize) -> World {
        World {
            entities: Vec::with_capacity(capacity),
            iterative_systems: Vec::new(),
            free_ents: VecDeque::with_capacity(capacity / 3)
        }
    }

    /// Registers a new iterative system, which will be called for every entity that
    /// matches its query on every tick.
    /// 
    /// # Examples
    /// ```
    /// use apollo_ecs::*;
    /// use apollo_ecs::systems::IterativeSystem;
    /// 
    /// struct SimpleSystem;
    /// 
    /// struct Phys {
    ///     mass: f32
    /// }
    /// struct Disabled;
    /// 
    /// impl SimpleSystem {
    ///     fn new() -> SimpleSystem {
    ///         SimpleSystem
    ///     }
    /// }
    /// 
    /// impl IterativeSystem for SimpleSystem {
    ///     fn get_query() -> EntityQuery {
    ///         EntityQuery::new(Matchers::with::<Phys>().without::<Disabled>())
    ///     }
    /// 
    ///     fn process(&mut self, ent: &EntityEditor, world: &World) {
    ///         let phys = ent.get::<Phys>().unwrap();
    ///         // Do something with phys here.
    ///     }
    /// }
    /// 
    /// let mut world = World::new();
    /// world.register_iterative_system(SimpleSystem::new());
    /// let ent = world.create_entity();
    /// world.edit(ent).unwrap().add(Phys { mass: 100.0 });
    /// ```
    pub fn register_iterative_system<T>(&mut self, system: T) where T: IterativeSystem + 'static {
        self.iterative_systems.push((RefCell::new(Box::new(system)), T::get_query()));
    } 

    /// Allocates space for a new entity and returns its ID
    pub fn create_entity(&mut self) -> Entity {
        if self.free_ents.len() > 0 {
            let ent = self.free_ents.pop_front().unwrap();
            let e = self.entities.get_mut(ent).unwrap();
            e.1.borrow_mut().truncate(0);
            e.0 = false;

            ent
        } else {
            self.entities.push((false, RefCell::new(Vec::with_capacity(12))));

            self.entities.len() - 1
        }
    }

    /// Removes an entity from the world and cleans up its components
    pub fn drop_entity(&mut self, ent: Entity) {
        if ent < self.entities.len() {
            let e = self.entities.get_mut(ent).unwrap();
            for comp in e.1.borrow().iter() {
                unsafe {
                    // Drop component memory
                    Box::from_raw(comp.1);
                }
            }

            e.0 = true;

            self.free_ents.push_back(ent);
        }
    }

    /// Edit an entity `ent`, if it exists.
    pub fn edit(&self, ent: Entity) -> Option<EntityEditor> {
        if let Some(e) = self.entities.get(ent) {
            if e.0 {
                None
            } else {
                Some(EntityEditor::new(ent, &e.1))
            }
        } else {
            None
        }
    }

    /// Filter all entities that match the provided query, and return an iterator
    /// to them.
    pub fn filter_entities<'a, 'q>(&'a self, query: &'q Query) -> QueryRunner<'a, 'q> {
        QueryRunner::new(&self.entities, query)
    }

    /// The main loop for a world. Calling `process` runs all ready systems in this world.
    pub fn process(&mut self) {
        for (ent, e) in self.entities.iter().enumerate() {
            if !e.0 {
                for sys in self.iterative_systems.iter() {
                    if sys.1.test(&e.1) {
                        sys.0.borrow_mut().process(&EntityEditor::new(ent, &e.1), self);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_create_entity() {
        let mut world = World::new();
        world.create_entity();

        assert_eq!(world.entities.len(), 1);
    }
    
    #[test]
    fn test_remove_entity() {
        let mut world = World::new();
        let ent = world.create_entity();

        assert_eq!(world.entities.len(), 1);

        world.drop_entity(ent);
        assert!(world.edit(ent).is_none());
        assert_eq!(world.entities.len(), 1);
        assert_eq!(world.entities.get(ent).unwrap().0, true);
    }
}