//! Mesh data structures and loading

use bytemuck::{Pod, Zeroable};
use glam::{Vec2, Vec3};
// use vulkano::buffer::BufferContents;

/// Vertex data for rendering
#[repr(C)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl Vertex {
    pub fn new(position: Vec3, normal: Vec3, tex_coords: Vec2) -> Self {
        Self {
            position: position.to_array(),
            normal: normal.to_array(),
            tex_coords: tex_coords.to_array(),
        }
    }
}

/// Mesh data
#[derive(Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub name: String,
}

impl Mesh {
    pub fn new(name: impl Into<String>, vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        Self {
            vertices,
            indices,
            name: name.into(),
        }
    }

    /// Create a simple cube mesh
    pub fn cube() -> Self {
        let vertices = vec![
            // Front face
            Vertex::new(Vec3::new(-0.5, -0.5,  0.5), Vec3::Z, Vec2::new(0.0, 1.0)),
            Vertex::new(Vec3::new( 0.5, -0.5,  0.5), Vec3::Z, Vec2::new(1.0, 1.0)),
            Vertex::new(Vec3::new( 0.5,  0.5,  0.5), Vec3::Z, Vec2::new(1.0, 0.0)),
            Vertex::new(Vec3::new(-0.5,  0.5,  0.5), Vec3::Z, Vec2::new(0.0, 0.0)),

            // Back face
            Vertex::new(Vec3::new(-0.5, -0.5, -0.5), Vec3::NEG_Z, Vec2::new(1.0, 1.0)),
            Vertex::new(Vec3::new(-0.5,  0.5, -0.5), Vec3::NEG_Z, Vec2::new(1.0, 0.0)),
            Vertex::new(Vec3::new( 0.5,  0.5, -0.5), Vec3::NEG_Z, Vec2::new(0.0, 0.0)),
            Vertex::new(Vec3::new( 0.5, -0.5, -0.5), Vec3::NEG_Z, Vec2::new(0.0, 1.0)),
        ];

        let indices = vec![
            0, 1, 2,  2, 3, 0,  // front
            4, 5, 6,  6, 7, 4,  // back
        ];

        Self::new("Cube", vertices, indices)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::{Vec2, Vec3};

    #[test]
    fn test_vertex_creation() {
        let pos = Vec3::new(1.0, 2.0, 3.0);
        let normal = Vec3::new(0.0, 1.0, 0.0);
        let tex_coords = Vec2::new(0.5, 0.5);

        let vertex = Vertex::new(pos, normal, tex_coords);

        assert_eq!(vertex.position, [1.0, 2.0, 3.0]);
        assert_eq!(vertex.normal, [0.0, 1.0, 0.0]);
        assert_eq!(vertex.tex_coords, [0.5, 0.5]);
    }

    #[test]
    fn test_vertex_default() {
        let vertex = Vertex::default();

        assert_eq!(vertex.position, [0.0, 0.0, 0.0]);
        assert_eq!(vertex.normal, [0.0, 0.0, 0.0]);
        assert_eq!(vertex.tex_coords, [0.0, 0.0]);
    }

    #[test]
    fn test_mesh_creation() {
        let vertices = vec![
            Vertex::new(Vec3::ZERO, Vec3::Y, Vec2::ZERO),
            Vertex::new(Vec3::X, Vec3::Y, Vec2::X),
        ];
        let indices = vec![0, 1, 2];

        let mesh = Mesh::new("TestMesh", vertices.clone(), indices.clone());

        assert_eq!(mesh.name, "TestMesh");
        assert_eq!(mesh.vertices, vertices);
        assert_eq!(mesh.indices, indices);
    }

    #[test]
    fn test_mesh_cube() {
        let cube = Mesh::cube();

        assert_eq!(cube.name, "Cube");
        assert_eq!(cube.vertices.len(), 8); // 8 vertices for a cube
        assert_eq!(cube.indices.len(), 12); // 2 faces * 6 indices per face

        // Check that all vertices have valid positions
        for vertex in &cube.vertices {
            // Positions should be in range [-0.5, 0.5] for a unit cube
            for &coord in &vertex.position {
                assert!(coord >= -0.5 && coord <= 0.5);
            }

            // Normals should be normalized
            let normal_length = (vertex.normal[0] * vertex.normal[0] +
                               vertex.normal[1] * vertex.normal[1] +
                               vertex.normal[2] * vertex.normal[2]).sqrt();
            assert!((normal_length - 1.0).abs() < 0.001);

            // Texture coordinates should be in range [0, 1]
            for &coord in &vertex.tex_coords {
                assert!(coord >= 0.0 && coord <= 1.0);
            }
        }

        // Check that indices are valid
        for &index in &cube.indices {
            assert!(index < cube.vertices.len() as u32);
        }
    }

    #[test]
    fn test_vertex_pod_zeroable() {
        // Test that Vertex implements Pod and Zeroable for GPU compatibility
        let vertex = Vertex::default();
        let bytes = bytemuck::bytes_of(&vertex);
        assert_eq!(bytes.len(), std::mem::size_of::<Vertex>());

        // Test zeroed vertex
        let zeroed = Vertex::zeroed();
        assert_eq!(zeroed.position, [0.0, 0.0, 0.0]);
        assert_eq!(zeroed.normal, [0.0, 0.0, 0.0]);
        assert_eq!(zeroed.tex_coords, [0.0, 0.0]);
    }
}
