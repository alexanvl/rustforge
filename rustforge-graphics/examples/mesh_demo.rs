//! Graphics example demonstrating mesh creation and manipulation

use rustforge_graphics::prelude::*;
use glam::{Vec2, Vec3};

fn main() {
    println!("RustForge Graphics - Mesh Demo");
    println!("==============================");

    // Create a simple triangle mesh
    let vertices = vec![
        Vertex::new(Vec3::new(0.0, 1.0, 0.0), Vec3::Z, Vec2::new(0.5, 0.0)),  // Top
        Vertex::new(Vec3::new(-1.0, -1.0, 0.0), Vec3::Z, Vec2::new(0.0, 1.0)), // Bottom left
        Vertex::new(Vec3::new(1.0, -1.0, 0.0), Vec3::Z, Vec2::new(1.0, 1.0)),  // Bottom right
    ];

    let indices = vec![0, 1, 2];

    let triangle_mesh = Mesh::new("Triangle", vertices, indices);
    println!("Created triangle mesh with {} vertices and {} indices",
             triangle_mesh.vertices.len(), triangle_mesh.indices.len());

    // Print vertex data
    for (i, vertex) in triangle_mesh.vertices.iter().enumerate() {
        println!("Vertex {}: pos={:?}, normal={:?}, uv={:?}",
                 i, vertex.position, vertex.normal, vertex.tex_coords);
    }

    // Create a cube mesh
    let cube_mesh = Mesh::cube();
    println!("\nCreated cube mesh with {} vertices and {} indices",
             cube_mesh.vertices.len(), cube_mesh.indices.len());

    // Analyze cube vertices
    let mut min_pos = Vec3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
    let mut max_pos = Vec3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);

    for vertex in &cube_mesh.vertices {
        let pos = Vec3::from_array(vertex.position);
        min_pos = min_pos.min(pos);
        max_pos = max_pos.max(pos);
    }

    println!("Cube bounds: min={:?}, max={:?}", min_pos, max_pos);
    println!("Cube size: {:?}", max_pos - min_pos);

    // Test vertex properties
    println!("\nVertex Properties");
    println!("================");

    let vertex = Vertex::new(
        Vec3::new(1.0, 2.0, 3.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec2::new(0.5, 0.5)
    );

    println!("Custom vertex: {:?}", vertex);
    println!("Position array: {:?}", vertex.position);
    println!("Normal array: {:?}", vertex.normal);
    println!("UV array: {:?}", vertex.tex_coords);

    // Test vertex size for GPU compatibility
    println!("Vertex size: {} bytes", std::mem::size_of::<Vertex>());
    println!("Vertex implements Pod trait for GPU compatibility");
    println!("Vertex implements Zeroable trait for GPU compatibility");
}

