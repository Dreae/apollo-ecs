use super::World;
use super::entities::Components;
use std::any::{Any, TypeId};

trait Condition {
    fn test(&self, components: Components) -> bool; 
}

pub struct QueryBuilder<'a> {
    world: &'a World,
    conditions: Vec<Box<Condition>>,
}

impl <'a> QueryBuilder<'a> {
    pub fn new(world: &'a World) -> QueryBuilder {
        QueryBuilder {
            world,
            conditions: Vec::new()
        }
    }

    pub fn with<T>(mut self) -> QueryBuilder<'a> where T: Any {
        self.conditions.push(Box::new(IsCondition { 
            ty: TypeId::of::<T>() 
        }));
        
        self
    }

    pub fn not<T>(mut self) -> QueryBuilder<'a> where T: Any {
        self.conditions.push(Box::new(NotCondition {
            ty: TypeId::of::<T>()
        }));

        self
    }

    pub fn and<T>(self) -> QueryBuilder<'a> where T: Any {
        let mut new_builder = QueryBuilder::new(self.world);
        new_builder.conditions.push(Box::new(AndCondition {
            left: Box::new(self.build()),
            right: Box::new(IsCondition { ty: TypeId::of::<T>() })
        }));

        new_builder
    }

    pub fn and_not<T>(self) -> QueryBuilder<'a> where T: Any {
        let mut new_builder = QueryBuilder::new(self.world);
        new_builder.conditions.push(Box::new(AndCondition {
            left: Box::new(self.build()),
            right: Box::new(NotCondition { ty: TypeId::of::<T>() })
        }));

        new_builder
    }

    pub fn or<T>(self) -> QueryBuilder<'a> where T: Any {
        let mut new_builder = QueryBuilder::new(self.world);
        new_builder.conditions.push(Box::new(OrCondition {
            left: Box::new(self.build()),
            right: Box::new(IsCondition { ty: TypeId::of::<T>() })
        }));

        new_builder
    }

    pub fn or_not<T>(self) -> QueryBuilder<'a> where T: Any {
        let mut new_builder = QueryBuilder::new(self.world);
        new_builder.conditions.push(Box::new(OrCondition {
            left: Box::new(self.build()),
            right: Box::new(NotCondition { ty: TypeId::of::<T>() })
        }));

        new_builder
    }

    fn build(self) -> Query {
        Query {
            conditions: self.conditions
        }
    }
}

pub struct Query {
    conditions: Vec<Box<Condition>>
}

impl Condition for Query {
    fn test(&self, components: Components) -> bool {
        for condition in self.conditions.iter() {
            if !condition.test(components.clone()) {
                return false;
            }
        }

        true
    }
}

struct IsCondition {
    ty: TypeId
}

struct NotCondition {
    ty: TypeId
}

struct AndCondition {
    left: Box<Condition>,
    right: Box<Condition>
}

struct OrCondition {
    left: Box<Condition>,
    right: Box<Condition>
}

impl Condition for IsCondition {
    fn test(&self, components: Components) -> bool {
        for &(ty, _) in components.lock().unwrap().iter() {
            if ty == self.ty {
                return true;
            }
        }

        false
    }
}

impl Condition for NotCondition {
    fn test(&self, components: Components) -> bool {
        for &(ty, _) in components.lock().unwrap().iter() {
            if ty == self.ty {
                return false;
            }
        }

        true
    }
}

impl Condition for AndCondition {
    fn test(&self, components: Components) -> bool {
        self.left.test(components.clone()) && self.right.test(components.clone())
    }
}

impl Condition for OrCondition {
    fn test(&self, components: Components) -> bool {
        self.left.test(components.clone()) || self.right.test(components.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_query_builder() {
        struct A;
        struct B;
        
        let world = World::new();
        let query = world.filter_entities().with::<A>().and::<B>().build();

        assert_eq!(query.conditions.len(), 1);

        assert!(query.test(Arc::new(Mutex::new(vec!((TypeId::of::<A>(), &mut 1), (TypeId::of::<B>(), &mut 2))))));
        assert_eq!(query.test(Arc::new(Mutex::new(vec!((TypeId::of::<A>(), &mut 1))))), false);
    }

    #[test]
    fn test_not_conditions() {
        struct A;
        struct B;
        struct C;
        
        let world = World::new();
        let query = world.filter_entities().not::<A>().and_not::<B>().build();

        assert_eq!(query.test(Arc::new(Mutex::new(vec!((TypeId::of::<A>(), &mut 1), (TypeId::of::<B>(), &mut 2))))), false);
        assert_eq!(query.test(Arc::new(Mutex::new(vec!((TypeId::of::<A>(), &mut 1))))), false);
        assert!(query.test(Arc::new(Mutex::new(vec!((TypeId::of::<C>(), &mut 1))))));
    }

    #[test]
    fn test_with_not() {
        struct A;
        struct B;
        struct C;
        
        let world = World::new();
        let query = world.filter_entities().with::<A>().not::<B>().build();

        assert_eq!(query.test(Arc::new(Mutex::new(vec!((TypeId::of::<A>(), &mut 1), (TypeId::of::<B>(), &mut 2))))), false);
        assert_eq!(query.test(Arc::new(Mutex::new(vec!((TypeId::of::<A>(), &mut 1))))), true);
        assert_eq!(query.test(Arc::new(Mutex::new(vec!((TypeId::of::<C>(), &mut 1))))), false);
    }

    #[test]
    fn test_or() {
        struct A;
        struct B;
        struct C;
        
        let world = World::new();
        let query = world.filter_entities().with::<A>().not::<B>().or::<C>().build();

        assert_eq!(query.test(Arc::new(Mutex::new(vec!((TypeId::of::<A>(), &mut 1), (TypeId::of::<B>(), &mut 2))))), false);
        assert_eq!(query.test(Arc::new(Mutex::new(vec!((TypeId::of::<A>(), &mut 1))))), true);
        assert_eq!(query.test(Arc::new(Mutex::new(vec!((TypeId::of::<C>(), &mut 1))))), true);
    }
}

#[cfg(all(feature = "nightly", test))]
mod benches {
    extern crate test;

    use self::test::Bencher;

    #[bench]
    fn bench_with_not(b: &mut Bencher) {
        b.iter(|| 2 + 2);
    }
}