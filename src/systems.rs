use super::{Entity, EntityQuery};

pub trait IterativeSystem {
    fn get_query() -> EntityQuery where Self: Sized;
    fn process(&mut self, ent: Entity);
}