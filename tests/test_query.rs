extern crate apollo_ecs;

use apollo_ecs::{World, Matchers};

#[test]
fn test_query_iter() {
    struct A;
    struct B;

    let mut world = World::new();

    world.create_entity();
    world.create_entity();
    
    let entity = world.create_entity();
    world.edit(entity).unwrap().add(A);

    let entity = world.create_entity();
    world.edit(entity).unwrap().add(A);
    world.edit(entity).unwrap().add(B);

    let mut i = 0;
    for _ in world.filter_entities(Matchers::with::<A>().with::<B>()) {
        i += 1;
    }
    assert_eq!(i, 1);

    i = 0;
    for _ in world.filter_entities(Matchers::with::<A>().or(Matchers::with::<B>())) {
        i += 1;
    }
    assert_eq!(i, 2);
}