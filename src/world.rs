use super::Entity;
use super::entities::{EntityEditor, Components};
use super::query::{QueryBuilder, QueryRunner};

pub struct World {
    pub(crate) entities: Vec<Components>,
}

impl World {
    pub fn new() -> World {
        World::with_capacity(524288)
    }

    pub fn with_capacity(capacity: usize) -> World {
        World {
            entities: Vec::with_capacity(capacity),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        self.entities.push(Vec::with_capacity(12));

        self.entities.len() - 1
    }

    pub fn edit(&mut self, ent: Entity) -> Option<EntityEditor> {
        if let Some(components) = self.entities.get_mut(ent) {
            Some(EntityEditor::new(ent, components))
        } else {
            None
        }
    }

    pub fn filter_entities(&mut self, builder: QueryBuilder) -> QueryRunner {
        QueryRunner::new(self, builder.build())
    }
}