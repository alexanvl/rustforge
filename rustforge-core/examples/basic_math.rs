//! Basic math example demonstrating Transform operations

use rustforge_core::prelude::*;

fn main() {
    println!("RustForge Core - Basic Math Example");
    println!("===================================");

    // Create a transform at origin
    let mut transform = Transform::default();
    println!("Default transform: {:?}", transform);

    // Move the transform
    transform.position = Vec3::new(5.0, 10.0, -3.0);
    println!("After moving: {:?}", transform);

    // Rotate the transform
    transform.rotation = Quat::from_rotation_y(std::f32::consts::PI / 4.0);
    println!("After rotating 45° around Y: {:?}", transform);

    // Scale the transform
    transform.scale = Vec3::new(2.0, 2.0, 2.0);
    println!("After scaling 2x: {:?}", transform);

    // Get direction vectors
    println!("Forward direction: {:?}", transform.forward());
    println!("Right direction: {:?}", transform.right());
    println!("Up direction: {:?}", transform.up());

    // Create transformation matrix
    let matrix = transform.matrix();
    println!("Transformation matrix:");
    println!("{:?}", matrix);

    // Test time management
    println!("\nTime Management Example");
    println!("=======================");

    let mut time = Time::new();
    println!("Initial time - Delta: {:.3}s, Total: {:.3}s",
             time.delta_seconds(), time.elapsed_seconds());

    // Simulate some time passing
    std::thread::sleep(std::time::Duration::from_millis(100));
    time.update();

    println!("After 100ms - Delta: {:.3}s, Total: {:.3}s",
             time.delta_seconds(), time.elapsed_seconds());

    // Test fixed timestep
    println!("\nFixed Timestep Example");
    println!("======================");

    let mut fixed_time = Time::new();
    let mut fixed_updates = 0;

    // Simulate multiple updates to accumulate time
    for _ in 0..10 {
        std::thread::sleep(std::time::Duration::from_millis(20)); // 20ms each
        fixed_time.update();

        while fixed_time.should_fixed_update() {
            fixed_updates += 1;
            println!("Fixed update #{}", fixed_updates);
        }
    }

    println!("Performed {} fixed updates over 10 iterations", fixed_updates);
}

