extern crate apollo_ecs;

use apollo_ecs::{EntityEditor, EntityQuery, World, Matchers};
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

    fn process(&mut self, _ent: EntityEditor) {
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
    world.edit(ent).unwrap().add(A);
    world.edit(ent).unwrap().add(B);
    
    let ent = world.create_entity();
    world.edit(ent).unwrap().add(A);
    world.edit(ent).unwrap().add(B);
    
    let ent = world.create_entity();
    world.edit(ent).unwrap().add(A);
    world.edit(ent).unwrap().add(B);
    world.edit(ent).unwrap().add(C);
    
    let ent = world.create_entity();
    world.edit(ent).unwrap().add(A);
    world.edit(ent).unwrap().add(B);

    world.process();

    unsafe {
        assert_eq!(MATCHED, 3);
    }
}