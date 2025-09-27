//! Graphics example demonstrating camera operations

use glam::{Quat, Vec3};
use rustforge_graphics::prelude::*;

fn main() {
    println!("RustForge Graphics - Camera Demo");
    println!("================================");

    // Create a perspective camera
    let mut perspective_camera = Camera::perspective(75.0, 16.0 / 9.0, 0.1, 1000.0);
    println!("Created perspective camera with 75° FOV");

    // Position the camera
    perspective_camera.transform.position = Vec3::new(0.0, 5.0, 10.0);
    perspective_camera.transform.rotation = Quat::from_rotation_x(-0.2);

    println!(
        "Camera position: {:?}",
        perspective_camera.transform.position
    );
    println!(
        "Camera rotation: {:?}",
        perspective_camera.transform.rotation
    );

    // Get camera matrices
    let view_matrix = perspective_camera.view_matrix();
    let proj_matrix = perspective_camera.projection_matrix();
    let view_proj_matrix = perspective_camera.view_projection_matrix();

    println!("View matrix determinant: {:.6}", view_matrix.determinant());
    println!(
        "Projection matrix determinant: {:.6}",
        proj_matrix.determinant()
    );
    println!(
        "View-projection matrix determinant: {:.6}",
        view_proj_matrix.determinant()
    );

    // Create an orthographic camera
    let mut ortho_camera = Camera::orthographic(20.0, 1.0, 0.1, 100.0);
    println!("\nCreated orthographic camera with size 20");

    ortho_camera.transform.position = Vec3::new(0.0, 0.0, 5.0);

    let ortho_view_matrix = ortho_camera.view_matrix();
    let ortho_proj_matrix = ortho_camera.projection_matrix();

    println!(
        "Orthographic view matrix determinant: {:.6}",
        ortho_view_matrix.determinant()
    );
    println!(
        "Orthographic projection matrix determinant: {:.6}",
        ortho_proj_matrix.determinant()
    );

    // Demonstrate camera movement
    println!("\nCamera Movement Demo");
    println!("===================");

    let mut camera = Camera::perspective(60.0, 1.0, 0.1, 100.0);
    camera.transform.position = Vec3::ZERO;

    // Move forward
    camera.transform.position += camera.transform.forward() * 5.0;
    println!(
        "After moving forward 5 units: {:?}",
        camera.transform.position
    );

    // Rotate and move right
    camera.transform.rotation *= Quat::from_rotation_y(std::f32::consts::PI / 2.0);
    camera.transform.position += camera.transform.right() * 3.0;
    println!(
        "After rotating 90° and moving right 3 units: {:?}",
        camera.transform.position
    );
}
