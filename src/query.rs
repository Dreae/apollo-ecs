use super::Entity;
use super::entities::{Component, Components};
use std::any::{Any, TypeId};
use std::cell::RefCell;

pub trait Condition {
    fn test(&self, components: &RefCell<Vec<Component>>) -> bool; 
}

pub struct Matchers;

impl Matchers {
    /// Special matcher that returns true for all entities.
    pub fn any() -> QueryBuilder {
        QueryBuilder::new().any()
    }

    /// Tests whether an entity has a component of type `T`
    pub fn with<T>() -> QueryBuilder where T: Any {
        QueryBuilder::new().with::<T>()
    }

    /// Tests whether an entity does not have a component of type `T`
    pub fn without<T>() -> QueryBuilder where T: Any {
        QueryBuilder::new().without::<T>()
    }

    /// True if the left-hand side of this expression, and `condition` 
    /// both test as true.
    pub fn and<T>(condition: T) -> QueryBuilder where T: Into<Box<Condition>> {
        QueryBuilder::new().and(condition)
    }

    /// True if the left-hand side of this expression, and `condition` 
    /// both test as false.
    pub fn and_not<T>(condition: T) -> QueryBuilder where T: Into<Box<Condition>> {
        QueryBuilder::new().and_not(condition)
    }

    /// True if either the left-hand side of this expression, or `condition` 
    /// test as true.
    pub fn or<T>(condition: T) -> QueryBuilder where T: Into<Box<Condition>> {
        QueryBuilder::new().or(condition)
    }

    /// True if either the left-hand side of this expression, or `condition` 
    /// test as false.
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

    pub fn any(mut self) -> QueryBuilder {
        self.conditions.push(Box::new(AnyCondition));

        self
    }

    /// Identical to [`Matchers.with`](struct.Matchers.html#method.with)
    pub fn with<T>(mut self) -> QueryBuilder where T: Any {
        self.conditions.push(Box::new(IsCondition { 
            ty: TypeId::of::<T>() 
        }));
        
        self
    }

    /// Identical to [`Matchers.without`](struct.Matchers.html#method.without)
    pub fn without<T>(mut self) -> QueryBuilder where T: Any {
        self.conditions.push(Box::new(IsNotCondition {
            ty: TypeId::of::<T>()
        }));

        self
    }

    /// Identical to [`Matchers.and`](struct.Matchers.html#method.and)
    pub fn and<T>(self, condition: T) -> QueryBuilder where T: Into<Box<Condition>> {
        let mut new_builder = QueryBuilder::new();
        new_builder.conditions.push(Box::new(AndCondition {
            left: Box::new(self.build()),
            right: condition.into()
        }));

        new_builder
    }

    /// Identical to [`Matchers.and_not`](struct.Matchers.html#method.and_not)
    pub fn and_not<T>(self, condition: T) -> QueryBuilder where T: Into<Box<Condition>> {
        let mut new_builder = QueryBuilder::new();
        new_builder.conditions.push(Box::new(AndCondition {
            left: Box::new(self.build()),
            right: Box::new(NotCondition { cond: condition.into() })
        }));

        new_builder
    }

    /// Identical to [`Matchers.or`](struct.Matchers.html#method.or)
    pub fn or<T>(self, condition: T) -> QueryBuilder where T: Into<Box<Condition>> {
        let mut new_builder = QueryBuilder::new();
        new_builder.conditions.push(Box::new(OrCondition {
            left: Box::new(self.build()),
            right: condition.into()
        }));

        new_builder
    }

    /// Identical to [`Matchers.or_not`](struct.Matchers.html#method.or_not)
    pub fn or_not<T>(self, condition: T) -> QueryBuilder where T: Into<Box<Condition>> {
        let mut new_builder = QueryBuilder::new();
        new_builder.conditions.push(Box::new(OrCondition {
            left: Box::new(self.build()),
            right: Box::new(NotCondition { cond: condition.into() })
        }));

        new_builder
    }

    /// Consumes this `QueryBuilder` and returns a finalized [`EntityQuery`](struct.EntityQuery.html)
    pub fn build(self) -> Query {
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

pub struct QueryRunner<'world, 'query> {
    ents: &'world Vec<RefCell<Components>>,
    query: &'query Query,
}

impl <'world, 'query> QueryRunner<'world, 'query> {
    pub fn new(ents: &'world Vec<RefCell<Components>>, query: &'query Query) -> QueryRunner<'world, 'query> {
        QueryRunner {
            ents,
            query,
        }
    }
}

impl <'world, 'query> IntoIterator for QueryRunner<'world, 'query> {
    type Item = Entity;
    type IntoIter = QueryRunnerIter<'world, 'query>;

    fn into_iter(self) -> Self::IntoIter {
        let ents = self.ents;
        QueryRunnerIter {
            query: self.query,
            ents: ents,
            index: 0
        }
    }
}

pub struct QueryRunnerIter<'world, 'query> {
    ents: &'world Vec<RefCell<Components>>,
    query: &'query Query,
    index: usize
}

impl <'world, 'query> Iterator for QueryRunnerIter<'world, 'query> {
    type Item = Entity;
    fn next(&mut self) -> Option<Self::Item> {
        for idx in self.index..self.ents.len() {
            if self.query.test(self.ents.get(idx).unwrap()) {
                self.index = idx + 1;

                return Some(idx)
            }
            
        }
        
        None
    }
}

/// Represents a set of rules for filtering entities before
/// they are passed into a system as part of a world tick
pub struct Query {
    conditions: Vec<Box<Condition>>
}

impl Query {
    pub fn new(builder: QueryBuilder) -> Query {
        builder.build()
    }
}

impl Condition for Query {
    fn test(&self, components: &RefCell<Vec<Component>>) -> bool {
        for condition in self.conditions.iter() {
            if !condition.test(components) {
                return false;
            }
        }

        true
    }
}

struct AnyCondition;

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

impl Condition for AnyCondition {
    fn test(&self, _components: &RefCell<Vec<Component>>) -> bool {
        true
    }
}

impl Condition for IsCondition {
    fn test(&self, components: &RefCell<Vec<Component>>) -> bool {
        for &(ty, _) in components.borrow().iter() {
            if ty == self.ty {
                return true;
            }
        }

        false
    }
}

impl Condition for IsNotCondition {
    fn test(&self, components: &RefCell<Vec<Component>>) -> bool {
        for &(ty, _) in components.borrow().iter() {
            if ty == self.ty {
                return false;
            }
        }

        true
    }
}

impl Condition for AndCondition {
    fn test(&self, components: &RefCell<Vec<Component>>) -> bool {
        self.left.test(components) && self.right.test(components)
    }
}

impl Condition for OrCondition {
    fn test(&self, components: &RefCell<Vec<Component>>) -> bool {
        self.left.test(components) || self.right.test(components)
    }
}

impl Condition for NotCondition {
    fn test(&self, components: &RefCell<Vec<Component>>) -> bool {
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

        assert!(query.test(&RefCell::new(vec!((TypeId::of::<A>(), &mut 1 as *mut Any), (TypeId::of::<B>(), &mut 2 as *mut Any)))));
        assert_eq!(query.test(&RefCell::new(vec!((TypeId::of::<A>(), &mut 1 as *mut Any)))), false);
    }

    #[test]
    fn test_any() {
        struct A;
        struct B;

        let query = Matchers::any().build();

        assert_eq!(query.test(&RefCell::new(vec!((TypeId::of::<A>(), &mut 1 as *mut Any), (TypeId::of::<B>(), &mut 2 as *mut Any)))), true);
        assert_eq!(query.test(&RefCell::new(vec!((TypeId::of::<A>(), &mut 1 as *mut Any)))), true);
        assert_eq!(query.test(&RefCell::new(vec!((TypeId::of::<B>(), &mut 1 as *mut Any)))), true);
    }

    #[test]
    fn test_not_conditions() {
        struct A;
        struct B;
        struct C;
        
        let query = Matchers::without::<A>().and_not(Matchers::with::<B>()).build();

        assert_eq!(query.test(&RefCell::new(vec!((TypeId::of::<A>(), &mut 1 as *mut Any), (TypeId::of::<B>(), &mut 2 as *mut Any)))), false);
        assert_eq!(query.test(&RefCell::new(vec!((TypeId::of::<A>(), &mut 1 as *mut Any)))), false);
        assert_eq!(query.test(&RefCell::new(vec!((TypeId::of::<C>(), &mut 1 as *mut Any)))), true);
    }

    #[test]
    fn test_with_not() {
        struct A;
        struct B;
        struct C;
        
        let query = Matchers::with::<A>().without::<B>().build();

        assert_eq!(query.test(&RefCell::new(vec!((TypeId::of::<A>(), &mut 1 as *mut Any), (TypeId::of::<B>(), &mut 2 as *mut Any)))), false);
        assert_eq!(query.test(&RefCell::new(vec!((TypeId::of::<A>(), &mut 1 as *mut Any)))), true);
        assert_eq!(query.test(&RefCell::new(vec!((TypeId::of::<C>(), &mut 1 as *mut Any)))), false);
    }

    #[test]
    fn test_or() {
        struct A;
        struct B;
        struct C;
        
        let query = Matchers::with::<A>().or(Matchers::with::<B>()).build();
        assert_eq!(query.test(&RefCell::new(vec!((TypeId::of::<A>(), &mut 1 as *mut Any)))), true);
        assert_eq!(query.test(&RefCell::new(vec!((TypeId::of::<B>(), &mut 1 as *mut Any)))), true);

        let query = Matchers::with::<A>().without::<B>().or(Matchers::with::<C>()).build();

        assert_eq!(query.test(&RefCell::new(vec!((TypeId::of::<A>(), &mut 1 as *mut Any), (TypeId::of::<B>(), &mut 2 as *mut Any)))), false);
        assert_eq!(query.test(&RefCell::new(vec!((TypeId::of::<A>(), &mut 1 as *mut Any)))), true);
        assert_eq!(query.test(&RefCell::new(vec!((TypeId::of::<C>(), &mut 1 as *mut Any)))), true);
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
            query.test(&RefCell::new(vec!((TypeId::of::<A>(), &mut test::black_box(1) as *mut Any), (TypeId::of::<B>(), &mut test::black_box(2) as *mut Any))));
        });
    }

    #[bench]
    fn bench_with(b: &mut Bencher) {
        struct A;
        struct B;
        
        let query = Matchers::with::<A>().build();

        b.iter(|| {
            query.test(&RefCell::new(vec!((TypeId::of::<A>(), &mut test::black_box(1) as *mut Any), (TypeId::of::<B>(), &mut test::black_box(2) as *mut Any))));
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
            query.test(&RefCell::new(vec!((TypeId::of::<A>(), &mut test::black_box(1) as *mut Any), (TypeId::of::<B>(), &mut test::black_box(2) as *mut Any), (TypeId::of::<C>(), &mut test::black_box(3) as *mut Any))));
        });
    }
}