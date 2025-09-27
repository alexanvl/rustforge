//! Physics example demonstrating physics world operations

use glam::Vec3;
use rapier3d::prelude::ColliderBuilder;
use rustforge_physics::prelude::*;
use rustforge_physics::rigid_body::create_rigid_body;
use specs::{Builder, WorldExt};

fn main() {
    println!("RustForge Physics - Physics World Demo");
    println!("======================================");

    // Create a physics world
    let mut physics_world = PhysicsWorld::new(Vec3::new(0.0, -9.81, 0.0));
    println!(
        "Created physics world with gravity: {:?}",
        physics_world.gravity
    );
    println!(
        "Fixed timestep: {:.3}s",
        physics_world.integration_parameters.dt
    );

    // Create some entities for testing
    let mut specs_world = specs::World::new();
    let entity1 = specs_world.create_entity().build();
    let entity2 = specs_world.create_entity().build();
    let entity3 = specs_world.create_entity().build();

    println!(
        "\nCreated entities: {:?}, {:?}, {:?}",
        entity1, entity2, entity3
    );

    // Add rigid bodies to the physics world
    println!("\nAdding Rigid Bodies");
    println!("==================");

    // Add a dynamic body (falling object)
    let dynamic_body = create_rigid_body(
        RigidBodyType::Dynamic,
        Vec3::new(0.0, 10.0, 0.0),
        glam::Quat::IDENTITY,
    )
    .build();
    let dynamic_handle = physics_world.add_rigid_body(entity1, dynamic_body);
    println!(
        "Added dynamic body for entity {:?} with handle {:?}",
        entity1, dynamic_handle
    );

    // Add a static body (ground)
    let static_body = create_rigid_body(
        RigidBodyType::Static,
        Vec3::new(0.0, -1.0, 0.0),
        glam::Quat::IDENTITY,
    )
    .build();
    let static_handle = physics_world.add_rigid_body(entity2, static_body);
    println!(
        "Added static body for entity {:?} with handle {:?}",
        entity2, static_handle
    );

    // Add a kinematic body (moving platform)
    let kinematic_body = create_rigid_body(
        RigidBodyType::Kinematic,
        Vec3::new(5.0, 2.0, 0.0),
        glam::Quat::IDENTITY,
    )
    .build();
    let kinematic_handle = physics_world.add_rigid_body(entity3, kinematic_body);
    println!(
        "Added kinematic body for entity {:?} with handle {:?}",
        entity3, kinematic_handle
    );

    // Add colliders to the bodies
    println!("\nAdding Colliders");
    println!("================");

    // Add a sphere collider to the dynamic body
    let sphere_collider = ColliderBuilder::ball(1.0)
        .density(1.0)
        .friction(0.5)
        .restitution(0.3)
        .build();
    let sphere_handle = physics_world.add_collider(entity1, sphere_collider, dynamic_handle);
    println!(
        "Added sphere collider for entity {:?} with handle {:?}",
        entity1, sphere_handle
    );

    // Add a box collider to the static body
    let box_collider = ColliderBuilder::cuboid(10.0, 0.5, 10.0)
        .friction(0.7)
        .restitution(0.1)
        .build();
    let box_handle = physics_world.add_collider(entity2, box_collider, static_handle);
    println!(
        "Added box collider for entity {:?} with handle {:?}",
        entity2, box_handle
    );

    // Add a capsule collider to the kinematic body
    let capsule_collider = ColliderBuilder::capsule_y(1.0, 0.5)
        .friction(0.6)
        .restitution(0.2)
        .build();
    let capsule_handle = physics_world.add_collider(entity3, capsule_collider, kinematic_handle);
    println!(
        "Added capsule collider for entity {:?} with handle {:?}",
        entity3, capsule_handle
    );

    // Check world state
    println!("\nWorld State");
    println!("===========");
    println!("Rigid bodies: {}", physics_world.rigid_body_set.len());
    println!("Colliders: {}", physics_world.collider_set.len());

    // Simulate physics
    println!("\nPhysics Simulation");
    println!("==================");

    // Get initial position of the dynamic body
    let initial_pos = physics_world
        .get_rigid_body(dynamic_handle)
        .unwrap()
        .translation();
    println!("Initial position of dynamic body: {:?}", initial_pos);

    // Step physics multiple times
    for i in 1..=5 {
        physics_world.step();
        let current_pos = physics_world
            .get_rigid_body(dynamic_handle)
            .unwrap()
            .translation();
        let velocity = physics_world
            .get_rigid_body(dynamic_handle)
            .unwrap()
            .linvel();
        println!(
            "Step {}: position={:?}, velocity={:?}",
            i, current_pos, velocity
        );
    }

    // Test entity to handle mapping
    println!("\nEntity to Handle Mapping");
    println!("========================");

    let entities = vec![entity1, entity2, entity3];
    for entity in entities {
        if let Some(body_handle) = physics_world.get_rigid_body_handle(entity) {
            if let Some(body) = physics_world.get_rigid_body(body_handle) {
                println!(
                    "Entity {:?} -> Body {:?} at position {:?}",
                    entity,
                    body_handle,
                    body.translation()
                );
            }
        }
    }

    // Test default physics world
    println!("\nDefault Physics World");
    println!("====================");

    let default_world = PhysicsWorld::default();
    println!("Default gravity: {:?}", default_world.gravity);
    println!(
        "Default timestep: {:.3}s",
        default_world.integration_parameters.dt
    );
}
