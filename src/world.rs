use super::Entity;
use super::query::{Query, Condition};
use super::systems::IterativeSystem;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::any::{Any, TypeId};

pub type Components = Vec<Component>;
pub type Component = (TypeId, *mut Any);

/// The world contains all entities and their components and delegates
/// their processing to systems.
pub struct World {
    pub(crate) entities: Vec<RefCell<Components>>,
    valid_ents: Vec<bool>,
    iterative_systems: Vec<(RefCell<Box<IterativeSystem>>, Query)>,
    free_ents: VecDeque<Entity>,
    dead_ents: RefCell<VecDeque<Entity>>
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
            free_ents: VecDeque::with_capacity(capacity / 3),
            dead_ents: RefCell::new(VecDeque::with_capacity(capacity / 3)),
            valid_ents: vec![false; capacity]
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
    ///     fn process(&mut self, ent: Entity, world: &World) {
    ///         let phys = world.get_component::<Phys>(ent).unwrap();
    ///         // Do something with phys here.
    ///     }
    /// }
    /// 
    /// let mut world = World::new();
    /// world.register_iterative_system(SimpleSystem::new());
    /// let ent = world.create_entity();
    /// world.add_component(ent, Phys { mass: 100.0 });
    /// ```
    pub fn register_iterative_system<T>(&mut self, system: T) where T: IterativeSystem + 'static {
        self.iterative_systems.push((RefCell::new(Box::new(system)), T::get_query()));
    } 

    /// Allocates space for a new entity and returns its ID
    pub fn create_entity(&mut self) -> Entity {
        if self.free_ents.len() > 0 {
            let ent = self.free_ents.pop_front().unwrap();
            let e = self.entities.get_mut(ent).unwrap();
            e.borrow_mut().truncate(0);
            self.valid_ents[ent] = false;

            ent
        } else {
            let ent = self.entities.len();
            self.entities.push(RefCell::new(Vec::with_capacity(12)));
            self.valid_ents[ent] = true;

            ent
        }
    }

    /// Removes an entity from the world and cleans up its components
    pub fn drop_entity(&mut self, ent: Entity) {
        if ent < self.entities.len() {
            let e = self.entities.get_mut(ent).unwrap();
            for comp in e.borrow().iter() {
                unsafe {
                    // Drop component memory
                    Box::from_raw(comp.1);
                }
            }

            self.valid_ents[ent] = false;

            self.free_ents.push_back(ent);
        }
    }

    /// Schedules an entity to be removed from the world on the next tick
    pub fn remove_entity(&self, ent: Entity) {
        if ent < self.entities.len() {
            self.dead_ents.borrow_mut().push_back(ent);
        }
    }

    /// Add a component of type `T` to entity `ent` and returns whether or not
    /// the operation was successful.
    pub fn add_component<T: Any>(&self, ent: Entity, component: T) -> bool {
        match self.valid_ents.get(ent) {
            Some(&true) => {
                let ty = TypeId::of::<T>();
                let mut components = self.entities[ent].borrow_mut();

                components.push((ty, Box::into_raw(Box::new(component))));
                
                true
            },
            _ => false
        }
    }

    /// Get the component of type `T` from entity `ent`
    pub fn get_component<T: Any>(&self, ent: Entity) -> Option<&mut T> {
        match self.valid_ents.get(ent) {
            Some(&true) => {
                let ty = TypeId::of::<T>();
                let components = &self.entities[ent];
                for &(comp_ty, ptr) in components.borrow().iter() {
                    if comp_ty == ty {
                        unsafe {
                            return Some(&mut *(ptr as *mut T));
                        }
                    }
                }

                None
            },
            _ => None
        }

    }

    /// Check whether entity `ent` has a component of type `T`
    pub fn has_component<T: Any>(&self, ent: Entity) -> bool {
        match self.valid_ents.get(ent) {
            Some(&true) => {
                let ty = TypeId::of::<T>();
                let components = &self.entities[ent];
                for &(comp_ty, _) in components.borrow().iter() {
                    if comp_ty == ty {
                        return true;
                    }
                }

                false

            },
            _ => false
        }
    }

    /// The main loop for a world. Calling `process` runs all ready systems in this world.
    pub fn process(&mut self) {
        for (ent, e) in self.entities.iter().enumerate() {
            if self.valid_ents[ent] {
                for sys in self.iterative_systems.iter() {
                    if sys.1.test(&e) {
                        sys.0.borrow_mut().process(ent, self);
                    }
                }
            }
        }

        if self.dead_ents.borrow().len() > 0 {
            loop  {
                let dead_ent = self.dead_ents.borrow_mut().pop_front();
                if dead_ent.is_some() {
                    self.drop_entity(dead_ent.unwrap());
                } else {
                    return;
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
    fn test_drop_entity() {
        let mut world = World::new();
        let ent = world.create_entity();

        assert_eq!(world.entities.len(), 1);

        world.drop_entity(ent);
        assert_eq!(world.entities.len(), 1);
        assert_eq!(world.valid_ents[ent], false);
    }

    #[test]
    fn test_remove_entity() {
        let mut world = World::new();
        let ent = world.create_entity();

        assert_eq!(world.entities.len(), 1);

        world.remove_entity(ent);
        world.process();
        assert_eq!(world.entities.len(), 1);
        assert_eq!(world.valid_ents[ent], false);
    }
}