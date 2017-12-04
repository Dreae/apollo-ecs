#![cfg(feature = "nightly")]
#![feature(test)]

extern crate test;
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

    fn process(&mut self, ent: &EntityEditor, _world: &World) {
        let pos = ent.get::<Position>().unwrap();

        pos.x += 1.0;
        pos.y -= 1.0;
    }
}

impl IterativeSystem for TestSystem2 {
    fn get_query() -> EntityQuery {
        EntityQuery::new(Matchers::with::<Position>())
    }

    fn process(&mut self, ent: &EntityEditor, _world: &World) {
        let pos = ent.get::<Position>().unwrap();

        pos.x -= 1.0;
        pos.y += 1.0;
    }
}

impl IterativeSystem for TestSystem3 {
    fn get_query() -> EntityQuery {
        EntityQuery::new(Matchers::with::<Position>())
    }

    fn process(&mut self, ent: &EntityEditor, _world: &World) {
        let pos = ent.get::<Position>().unwrap();

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
        
        world.edit(ent).unwrap().add(Position {
            x: 0.0,
            y: 0.0
        });
    }

    b.iter(|| {
        world.process()
    });
}