use super::{EntityEditor, EntityQuery};

pub trait IterativeSystem {
    fn get_query() -> EntityQuery where Self: Sized;
    // TODO: Shound't take an EntityEditor
    fn process(&mut self, ent: EntityEditor);
}