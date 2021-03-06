#![cfg(feature = "nightly")]
#![feature(test)]

extern crate test;

#[cfg(feature = "profiler")]
extern crate cpuprofiler;
#[cfg(feature = "profiler")]
use cpuprofiler::PROFILER;

extern crate apollo_ecs;

use test::Bencher;


use apollo_ecs::*;
use apollo_ecs::systems::IterativeSystem;

struct TestSystem1;
struct TestSystem2;
struct TestSystem3;

struct Position {
    x: f32,
    y: f32
}

impl IterativeSystem for TestSystem1 {
    fn get_query() -> EntityQuery {
        EntityQuery::new(Matchers::with::<Position>())
    }

    fn process(&mut self, ent: Entity, world: &World) {
        let pos = world.get_component::<Position>(ent).unwrap();

        pos.x += 1.0;
        pos.y -= 1.0;
    }
}

impl IterativeSystem for TestSystem2 {
    fn get_query() -> EntityQuery {
        EntityQuery::new(Matchers::with::<Position>())
    }

    fn process(&mut self, ent: Entity, world: &World) {
        let pos = world.get_component::<Position>(ent).unwrap();

        pos.x -= 1.0;
        pos.y += 1.0;
    }
}

impl IterativeSystem for TestSystem3 {
    fn get_query() -> EntityQuery {
        EntityQuery::new(Matchers::with::<Position>())
    }

    fn process(&mut self, ent: Entity, world: &World) {
        let pos = world.get_component::<Position>(ent).unwrap();

        pos.x += 1.0;
        pos.y -= 1.0;
    }
}

#[bench]
fn bench_16384_ents(b: &mut Bencher) {
    let mut world = World::new();
    world.register_iterative_system(TestSystem1);
    world.register_iterative_system(TestSystem2);
    world.register_iterative_system(TestSystem3);

    for _ in 0..16384 {
        let ent = world.create_entity();
        
        world.add_component(ent, Position {
            x: 0.0,
            y: 0.0
        });
    }

    #[cfg(feature = "profiler")]
    {
        PROFILER.lock().unwrap().start("bench_16384.prof").unwrap();
    }
    b.iter(|| {
        world.process()
    });

    #[cfg(feature = "profiler")]
    {
        PROFILER.lock().unwrap().stop().unwrap();
    }
}