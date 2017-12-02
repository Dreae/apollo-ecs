#![cfg_attr(feature = "nightly", feature(test))]

mod world;
mod entities;
mod query;
pub mod systems;

pub type Entity = usize;

pub use world::World;
pub use query::{Matchers, Query as EntityQuery};