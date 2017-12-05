use super::Entity;
use std::any::{Any, TypeId};
use std::cell::RefCell;

pub type Components = Vec<Component>;
pub type Component = (TypeId, *mut Any);

pub struct EntityEditor<'a> {
    ent: Entity,
    components: &'a RefCell<Components>
}

impl <'a> EntityEditor<'a> {
    pub fn new(ent: Entity, components: &'a RefCell<Components>) -> EntityEditor {
        EntityEditor {
            ent,
            components
        }
    }

    pub fn add<T>(self, component: T) -> EntityEditor<'a> where T: Any {
        self.components.borrow_mut().push((TypeId::of::<T>(), Box::into_raw(Box::new(component))));
        
        self
    }

    pub fn has<T>(&self) -> bool where T: Any {
        let ty = TypeId::of::<T>();
        for &(comp_ty, _) in self.components.borrow().iter() {
            if comp_ty == ty {
                return true;
            }
        }

        false
    }

    pub fn get<T>(&self) -> Option<&mut T> where T: Any {
        let ty = TypeId::of::<T>();
        for &(comp_ty, ptr) in self.components.borrow().iter() {
            if comp_ty == ty {
                unsafe {
                    return Some(&mut *(ptr as *mut T));
                }
            }
        }

        None
    }
}

impl <'a> Into<Entity> for EntityEditor<'a> {
    fn into(self) -> Entity {
        self.ent
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {
        struct A;

        let comps = RefCell::new(Vec::new());
        {
            let editor = EntityEditor::new(1, &comps);
            editor.add(A);
        }
        
        assert_eq!(comps.borrow().len(), 1);
    }
}