extern crate apollo_ecs;

use apollo_ecs::{Entity, EntityQuery, World, Matchers};
use apollo_ecs::systems::IterativeSystem;

struct TestSystem;

struct A;
struct B;
struct C;

static mut MATCHED: i32 = 0;

impl IterativeSystem for TestSystem {
    fn get_query() -> EntityQuery {
        EntityQuery::new(Matchers::with::<A>().with::<B>().and(Matchers::without::<C>()))
    }

    fn process(&mut self, _ent: Entity, _world: &World) {
        unsafe {
            MATCHED += 1;
        }
    }
}

#[test]
fn test_iterative_system() {
    let mut world = World::new();
    world.register_iterative_system(TestSystem);
    
    let ent = world.create_entity();
    world.add_component(ent, A);
    world.add_component(ent, B);
    
    let ent = world.create_entity();
    world.add_component(ent, A);
    world.add_component(ent, B);
    
    let ent = world.create_entity();
    world.add_component(ent, A);
    world.add_component(ent, B);
    world.add_component(ent, C);
    
    let ent = world.create_entity();
    world.add_component(ent, A);
    world.add_component(ent, B);

    world.process();

    unsafe {
        assert_eq!(MATCHED, 3);
    }
}