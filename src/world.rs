use super::Entity;
use super::entities::{EntityEditor, Components};
use super::query::{Query, QueryRunner};
use super::systems::IterativeSystem;

use std::ops::DerefMut;
use std::cell::RefCell;

pub struct World {
    pub(crate) entities: Vec<RefCell<Components>>,
    iterative_systems: Vec<(Box<IterativeSystem>, Query)>
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
            iterative_systems: Vec::new()
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
    ///     fn process(&mut self, ent: EntityEditor) {
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
        self.iterative_systems.push((Box::new(system), T::get_query()));
    } 

    /// Allocates space for a new entity and returns its ID
    pub fn create_entity(&mut self) -> Entity {
        self.entities.push(RefCell::new(Vec::with_capacity(12)));

        self.entities.len() - 1
    }

    /// Removes an entity from the world and cleans up its components
    pub fn drop_entity(&mut self, ent: Entity) {
        if ent < self.entities.len() {
            let components = self.entities.remove(ent);
            for comp in components.borrow().iter() {
                unsafe {
                    // Drop component memory
                    Box::from_raw(comp.1);
                }
            }
        }
    }

    /// Edit an entity `ent`, if it exists.
    pub fn edit(&mut self, ent: Entity) -> Option<EntityEditor> {
        if let Some(components) = self.entities.get_mut(ent) {
            Some(EntityEditor::new(ent, components))
        } else {
            None
        }
    }

    /// Filter all entities that match the provided query, and return an iterator
    /// to them.
    pub fn filter_entities<'a, 'q>(&'a mut self, query: &'q Query) -> QueryRunner<'a, 'q> {
        QueryRunner::new(&self.entities, query)
    }

    /// The main loop for a world. Calling `process` runs all ready systems in this world.
    pub fn process(&mut self) {
        for sys in self.iterative_systems.iter_mut() {
            for ent in QueryRunner::new(&self.entities, &sys.1) {
                sys.0.deref_mut().process(EntityEditor::new(ent, self.entities.get(ent).unwrap()));
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
        assert_eq!(world.entities.len(), 0);
    }
}