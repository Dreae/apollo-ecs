use super::{World, Entity};
use super::entities::Component;
use std::any::{Any, TypeId};

pub trait Condition {
    fn test(&self, components: &[Component]) -> bool; 
}

pub struct Matchers;

impl Matchers {
    pub fn with<T>() -> QueryBuilder where T: Any {
        QueryBuilder::new().with::<T>()
    }

    pub fn without<T>() -> QueryBuilder where T: Any {
        QueryBuilder::new().without::<T>()
    }

    pub fn and<T>(condition: T) -> QueryBuilder where T: Into<Box<Condition>> {
        QueryBuilder::new().and(condition)
    }

    pub fn and_not<T>(condition: T) -> QueryBuilder where T: Into<Box<Condition>> {
        QueryBuilder::new().and_not(condition)
    }

    pub fn or<T>(condition: T) -> QueryBuilder where T: Into<Box<Condition>> {
        QueryBuilder::new().or(condition)
    }

    pub fn or_not<T>(condition: T) -> QueryBuilder where T: Into<Box<Condition>> {
        QueryBuilder::new().or_not(condition)
    }
}

pub struct QueryBuilder {
    conditions: Vec<Box<Condition>>,
}

impl <'a> QueryBuilder {
    pub fn new() -> QueryBuilder {
        QueryBuilder {
            conditions: Vec::new()
        }
    }

    pub fn with<T>(mut self) -> QueryBuilder where T: Any {
        self.conditions.push(Box::new(IsCondition { 
            ty: TypeId::of::<T>() 
        }));
        
        self
    }

    pub fn without<T>(mut self) -> QueryBuilder where T: Any {
        self.conditions.push(Box::new(IsNotCondition {
            ty: TypeId::of::<T>()
        }));

        self
    }

    pub fn and<T>(self, condition: T) -> QueryBuilder where T: Into<Box<Condition>> {
        let mut new_builder = QueryBuilder::new();
        new_builder.conditions.push(Box::new(AndCondition {
            left: Box::new(self.build()),
            right: condition.into()
        }));

        new_builder
    }

    pub fn and_not<T>(self, condition: T) -> QueryBuilder where T: Into<Box<Condition>> {
        let mut new_builder = QueryBuilder::new();
        new_builder.conditions.push(Box::new(AndCondition {
            left: Box::new(self.build()),
            right: Box::new(NotCondition { cond: condition.into() })
        }));

        new_builder
    }

    pub fn or<T>(self, condition: T) -> QueryBuilder where T: Into<Box<Condition>> {
        let mut new_builder = QueryBuilder::new();
        new_builder.conditions.push(Box::new(OrCondition {
            left: Box::new(self.build()),
            right: condition.into()
        }));

        new_builder
    }

    pub fn or_not<T>(self, condition: T) -> QueryBuilder where T: Into<Box<Condition>> {
        let mut new_builder = QueryBuilder::new();
        new_builder.conditions.push(Box::new(OrCondition {
            left: Box::new(self.build()),
            right: Box::new(NotCondition { cond: condition.into() })
        }));

        new_builder
    }

    pub(crate) fn build(self) -> Query {
        Query {
            conditions: self.conditions
        }
    }
}

impl Into<Box<Condition>> for QueryBuilder {
    fn into(self) -> Box<Condition> {
        Box::new(self.build())
    }
}

pub struct QueryRunner<'a> {
    world: &'a World,
    query: Query
}

impl <'a> QueryRunner<'a> {
    pub fn new(world: &'a World, query: Query) -> QueryRunner<'a> {
        QueryRunner {
            world,
            query
        }
    }
}

impl <'a> IntoIterator for QueryRunner<'a> {
    type Item = Entity;
    type IntoIter = QueryRunnerIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let world = self.world;
        QueryRunnerIter {
            query: self.query,
            world: world,
            index: 0
        }
    }
}

pub struct QueryRunnerIter<'a> {
    world: &'a World,
    query: Query,
    index: usize
}

impl <'a> Iterator for QueryRunnerIter<'a> {
    type Item = Entity;
    fn next(&mut self) -> Option<Self::Item> {
        for idx in self.index..self.world.entities.len() {
            if let Some(components) = self.world.entities.get(idx) {
                if self.query.test(&*components) {
                    self.index = idx + 1;

                    return Some(idx)
                }
            }
        }
        
        None
    }
}

pub struct Query {
    conditions: Vec<Box<Condition>>
}


impl Condition for Query {
    fn test(&self, components: &[Component]) -> bool {
        for condition in self.conditions.iter() {
            if !condition.test(components) {
                return false;
            }
        }

        true
    }
}

struct IsCondition {
    ty: TypeId
}

struct IsNotCondition {
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

struct NotCondition {
    cond: Box<Condition>
}

impl Condition for IsCondition {
    fn test(&self, components: &[Component]) -> bool {
        for &(ty, _) in components.iter() {
            if ty == self.ty {
                return true;
            }
        }

        false
    }
}

impl Condition for IsNotCondition {
    fn test(&self, components: &[Component]) -> bool {
        for &(ty, _) in components.iter() {
            if ty == self.ty {
                return false;
            }
        }

        true
    }
}

impl Condition for AndCondition {
    fn test(&self, components: &[Component]) -> bool {
        self.left.test(components) && self.right.test(components)
    }
}

impl Condition for OrCondition {
    fn test(&self, components: &[Component]) -> bool {
        self.left.test(components) || self.right.test(components)
    }
}

impl Condition for NotCondition {
    fn test(&self, components: &[Component]) -> bool {
        !self.cond.test(components)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_builder() {
        struct A;
        struct B;
        
        let query = Matchers::with::<A>().with::<B>().build();

        assert!(query.test(&vec!((TypeId::of::<A>(), &mut 1 as *mut Any), (TypeId::of::<B>(), &mut 2 as *mut Any))));
        assert_eq!(query.test(&vec!((TypeId::of::<A>(), &mut 1 as *mut Any))), false);
    }

    #[test]
    fn test_not_conditions() {
        struct A;
        struct B;
        struct C;
        
        let query = Matchers::without::<A>().and_not(Matchers::with::<B>()).build();

        assert_eq!(query.test(&vec!((TypeId::of::<A>(), &mut 1 as *mut Any), (TypeId::of::<B>(), &mut 2 as *mut Any))), false);
        assert_eq!(query.test(&vec!((TypeId::of::<A>(), &mut 1 as *mut Any))), false);
        assert_eq!(query.test(&vec!((TypeId::of::<C>(), &mut 1 as *mut Any))), true);
    }

    #[test]
    fn test_with_not() {
        struct A;
        struct B;
        struct C;
        
        let query = Matchers::with::<A>().without::<B>().build();

        assert_eq!(query.test(&vec!((TypeId::of::<A>(), &mut 1 as *mut Any), (TypeId::of::<B>(), &mut 2 as *mut Any))), false);
        assert_eq!(query.test(&vec!((TypeId::of::<A>(), &mut 1 as *mut Any))), true);
        assert_eq!(query.test(&vec!((TypeId::of::<C>(), &mut 1 as *mut Any))), false);
    }

    #[test]
    fn test_or() {
        struct A;
        struct B;
        struct C;
        
        let query = Matchers::with::<A>().or(Matchers::with::<B>()).build();
        assert_eq!(query.test(&vec!((TypeId::of::<A>(), &mut 1 as *mut Any))), true);
        assert_eq!(query.test(&vec!((TypeId::of::<B>(), &mut 1 as *mut Any))), true);

        let query = Matchers::with::<A>().without::<B>().or(Matchers::with::<C>()).build();

        assert_eq!(query.test(&vec!((TypeId::of::<A>(), &mut 1 as *mut Any), (TypeId::of::<B>(), &mut 2 as *mut Any))), false);
        assert_eq!(query.test(&vec!((TypeId::of::<A>(), &mut 1 as *mut Any))), true);
        assert_eq!(query.test(&vec!((TypeId::of::<C>(), &mut 1 as *mut Any))), true);
    }
}

#[cfg(all(feature = "nightly", test))]
mod benches {
    extern crate test;

    use self::test::Bencher;
    use std::any::TypeId;

    use super::*;

    #[bench]
    fn bench_with_not_or(b: &mut Bencher) {
        struct A;
        struct B;
        struct C;

        let query = Matchers::with::<A>().without::<B>().or(Matchers::with::<C>()).build();
        
        b.iter(|| {
            query.test(&vec!((TypeId::of::<A>(), &mut test::black_box(1) as *mut Any), (TypeId::of::<B>(), &mut test::black_box(2) as *mut Any)));
        });
    }

    #[bench]
    fn bench_with(b: &mut Bencher) {
        struct A;
        struct B;
        
        let query = Matchers::with::<A>().build();

        b.iter(|| {
            query.test(&vec!((TypeId::of::<A>(), &mut test::black_box(1) as *mut Any), (TypeId::of::<B>(), &mut test::black_box(2) as *mut Any)));
        });
    }

    #[bench]
    fn bench_with_with_with_not(b: &mut Bencher) {
        struct A;
        struct B;
        struct C;
        struct D;
        
        let query = Matchers::with::<A>().with::<B>().with::<C>().without::<D>().build();

        b.iter(|| {
            query.test(&vec!((TypeId::of::<A>(), &mut test::black_box(1) as *mut Any), (TypeId::of::<B>(), &mut test::black_box(2) as *mut Any), (TypeId::of::<C>(), &mut test::black_box(3) as *mut Any)));
        });
    }
}