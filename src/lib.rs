use std::sync::{Arc, Mutex};
use std::any::{Any, TypeId};

mod world;
mod entities;

pub type Entity = usize;
pub type Components = Arc<Mutex<Vec<(TypeId, Box<Any>)>>>;

pub use world::World;