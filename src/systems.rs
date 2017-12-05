use super::{EntityEditor, EntityQuery, World};

/// An `IterativeSystem` iterates over all entities matching its
/// provided `EntityQuery` on every world tick.
pub trait IterativeSystem {
    /// Static method to return the query that will be used
    /// to filter the world's entities before they are passed
    /// to this system
    fn get_query() -> EntityQuery where Self: Sized;

    // TODO: Shound't take an EntityEditor
    /// The main loop for this system, `process` is called
    /// for every entity that matches this system's query
    /// on every world tick.
    fn process(&mut self, ent: &EntityEditor, world: &World);
}