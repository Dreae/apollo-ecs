#![cfg_attr(feature = "nightly", feature(test))]

mod world;
mod entities;
mod query;

pub type Entity = usize;

pub use world::World;
pub use query::Matchers;