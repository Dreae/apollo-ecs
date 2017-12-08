#![cfg_attr(feature = "nightly", feature(test))]

//! Apollo is a lightwieght entity-component-system framework written in pure
//! Rust. 
//! 
//! # Examples
//! 
//! ```no_run
//! use apollo_ecs::*;
//! use apollo_ecs::systems::IterativeSystem;
//! 
//! struct SimpleSystem;
//! 
//! struct Phys {
//!     mass: f32
//! }
//! struct Disabled;
//! 
//! impl SimpleSystem {
//!     fn new() -> SimpleSystem {
//!         SimpleSystem
//!     }
//! }
//! 
//! impl IterativeSystem for SimpleSystem {
//!     fn get_query() -> EntityQuery {
//!         EntityQuery::new(Matchers::with::<Phys>().without::<Disabled>())
//!     }
//! 
//!     fn process(&mut self, ent: &EntityEditor, world: &World) {
//!         let phys = ent.get::<Phys>().unwrap();
//!         // Do something with phys here.
//!     }
//! }
//! 
//! fn main() {
//!     let mut world = World::new();
//!     world.register_iterative_system(SimpleSystem::new());
//!     let ent = world.create_entity();
//!     world.edit(ent).unwrap().add(Phys { mass: 100.0 });
//!     
//!     // Represents the main game loop
//!     loop { 
//!         world.process();
//!     }
//! }
//! ```
//! 
//! To read more about ECS check [here](http://entity-systems.wikidot.com/)
mod world;
mod entities;
mod query;
mod bitvec;

/// Contains traits for implementing various modes of entity processing
/// in systems
pub mod systems;

/// An entity's ID
pub type Entity = usize;

pub use world::World;
pub use query::{Matchers, Query as EntityQuery};
pub use entities::EntityEditor;