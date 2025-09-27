//! Physics example demonstrating collider creation and properties

use glam::Vec3;
use rustforge_physics::prelude::*;

fn main() {
    println!("RustForge Physics - Collider Demo");
    println!("=================================");

    // Create different collider shapes
    let shapes = vec![
        (
            "Box",
            ColliderShape::Box {
                half_extents: Vec3::new(1.0, 2.0, 3.0),
            },
        ),
        ("Sphere", ColliderShape::Sphere { radius: 2.5 }),
        (
            "Capsule",
            ColliderShape::Capsule {
                half_height: 1.0,
                radius: 0.5,
            },
        ),
        (
            "Cylinder",
            ColliderShape::Cylinder {
                half_height: 2.0,
                radius: 1.0,
            },
        ),
        (
            "Cone",
            ColliderShape::Cone {
                half_height: 1.5,
                radius: 0.8,
            },
        ),
    ];

    println!("Creating colliders with different shapes:");
    for (name, shape) in shapes {
        let collider = shape
            .build_collider()
            .friction(0.5)
            .restitution(0.2)
            .build();

        println!(
            "{} collider: friction={:.1}, restitution={:.1}",
            name,
            collider.friction(),
            collider.restitution()
        );
    }

    // Create a convex mesh collider
    println!("\nConvex Mesh Collider");
    println!("===================");

    let cube_vertices = vec![
        Vec3::new(-1.0, -1.0, -1.0),
        Vec3::new(1.0, -1.0, -1.0),
        Vec3::new(1.0, 1.0, -1.0),
        Vec3::new(-1.0, 1.0, -1.0),
        Vec3::new(-1.0, -1.0, 1.0),
        Vec3::new(1.0, -1.0, 1.0),
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(-1.0, 1.0, 1.0),
    ];

    let convex_shape = ColliderShape::ConvexMesh {
        vertices: cube_vertices,
    };
    let convex_collider = convex_shape
        .build_collider()
        .friction(0.7)
        .restitution(0.3)
        .build();

    println!(
        "Convex mesh collider: friction={:.1}, restitution={:.1}",
        convex_collider.friction(),
        convex_collider.restitution()
    );

    // Create a triangle mesh collider
    println!("\nTriangle Mesh Collider");
    println!("=====================");

    let triangle_vertices = vec![
        Vec3::new(0.0, 1.0, 0.0),   // Top
        Vec3::new(-1.0, -1.0, 0.0), // Bottom left
        Vec3::new(1.0, -1.0, 0.0),  // Bottom right
    ];
    let triangle_indices = vec![[0, 1, 2]];

    let trimesh_shape = ColliderShape::TriMesh {
        vertices: triangle_vertices,
        indices: triangle_indices,
    };
    let trimesh_collider = trimesh_shape
        .build_collider()
        .friction(0.6)
        .restitution(0.1)
        .build();

    println!(
        "Triangle mesh collider: friction={:.1}, restitution={:.1}",
        trimesh_collider.friction(),
        trimesh_collider.restitution()
    );

    // Demonstrate collider properties
    println!("\nCollider Properties");
    println!("==================");

    let collider = ColliderShape::Sphere { radius: 1.0 }
        .build_collider()
        .friction(0.8)
        .restitution(0.4)
        .sensor(false)
        .density(2.0)
        .build();

    println!("Initial properties:");
    println!("  Friction: {:.1}", collider.friction());
    println!("  Restitution: {:.1}", collider.restitution());
    println!("  Is sensor: {}", collider.is_sensor());
    println!("  Density: {:.1}", collider.density());

    // Create a sensor collider
    let sensor_collider = ColliderShape::Sphere { radius: 5.0 }
        .build_collider()
        .sensor(true)
        .build();

    println!("\nSensor collider:");
    println!("  Is sensor: {}", sensor_collider.is_sensor());
    println!("  Friction: {:.1}", sensor_collider.friction());
}
