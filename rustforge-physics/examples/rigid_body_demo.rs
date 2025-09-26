//! Physics example demonstrating rigid body creation and properties

use rustforge_physics::prelude::*;
use rustforge_physics::rigid_body::create_rigid_body;
use glam::{Vec3, Quat};

fn main() {
    println!("RustForge Physics - Rigid Body Demo");
    println!("===================================");

    // Create different types of rigid bodies
    let position = Vec3::new(0.0, 5.0, 0.0);
    let rotation = Quat::from_rotation_y(std::f32::consts::PI / 4.0);

    println!("Creating rigid bodies at position {:?} with rotation {:?}", position, rotation);

    // Dynamic rigid body
    let dynamic_body = create_rigid_body(RigidBodyType::Dynamic, position, rotation).build();
    println!("\nDynamic rigid body:");
    println!("  Type: {:?}", dynamic_body.body_type());
    println!("  Position: {:?}", dynamic_body.translation());
    println!("  CCD enabled: {}", dynamic_body.is_ccd_enabled());
    println!("  Is sleeping: {}", dynamic_body.is_sleeping());

    // Static rigid body
    let static_body = create_rigid_body(RigidBodyType::Static, position, rotation).build();
    println!("\nStatic rigid body:");
    println!("  Type: {:?}", static_body.body_type());
    println!("  Position: {:?}", static_body.translation());
    println!("  CCD enabled: {}", static_body.is_ccd_enabled());

    // Kinematic rigid body
    let kinematic_body = create_rigid_body(RigidBodyType::Kinematic, position, rotation).build();
    println!("\nKinematic rigid body:");
    println!("  Type: {:?}", kinematic_body.body_type());
    println!("  Position: {:?}", kinematic_body.translation());
    println!("  CCD enabled: {}", kinematic_body.is_ccd_enabled());

    // Demonstrate rigid body type conversion
    println!("\nRigid Body Type Conversion");
    println!("=========================");

    let types = vec![
        RigidBodyType::Dynamic,
        RigidBodyType::Kinematic,
        RigidBodyType::Static,
    ];

    for body_type in types {
        let rapier_type: rapier3d::dynamics::RigidBodyType = body_type.into();
        println!("{:?} -> {:?}", body_type, rapier_type);
    }

    // Test different positions and rotations
    println!("\nPosition and Rotation Tests");
    println!("===========================");

    let test_cases = vec![
        ("Origin", Vec3::ZERO, Quat::IDENTITY),
        ("Offset", Vec3::new(10.0, -5.0, 3.0), Quat::IDENTITY),
        ("Rotated", Vec3::ZERO, Quat::from_rotation_x(std::f32::consts::PI / 2.0)),
        ("Both", Vec3::new(5.0, 10.0, -2.0), Quat::from_rotation_z(std::f32::consts::PI / 3.0)),
    ];

    for (name, pos, rot) in test_cases {
        let body = create_rigid_body(RigidBodyType::Dynamic, pos, rot).build();
        println!("{}: pos={:?}, rot={:?}", name, body.translation(), body.rotation());
    }

    // Demonstrate body properties
    println!("\nBody Properties");
    println!("==============");

    let body = create_rigid_body(
        RigidBodyType::Dynamic,
        Vec3::new(0.0, 10.0, 0.0),
        Quat::IDENTITY
    ).build();

    println!("Initial state:");
    println!("  Position: {:?}", body.translation());
    println!("  Linear velocity: {:?}", body.linvel());
    println!("  Angular velocity: {:?}", body.angvel());
    println!("  Mass: {:.2}", body.mass());
    println!("  Center of mass: {:?}", body.center_of_mass());
}

