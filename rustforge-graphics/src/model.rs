//! High-level model abstraction for common 3D shapes

use crate::vertex::PositionNormal;
use wgpu::util::DeviceExt;
use bytemuck::{Pod, Zeroable};

/// Standard uniforms for model rendering
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct ModelUniforms {
    pub model: [[f32; 4]; 4],
    pub view: [[f32; 4]; 4],
    pub projection: [[f32; 4]; 4],
    pub light_position: [f32; 3],
    pub _padding1: f32,
    pub camera_position: [f32; 3],
    pub _padding2: f32,
}

/// A 3D model with basic rendering capabilities
pub struct Model {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    pub num_indices: u32,
    name: String,
}

impl Model {
    /// Create a model from vertices and indices
    pub fn new(
        device: &wgpu::Device,
        name: impl Into<String>,
        vertices: Vec<PositionNormal>,
        indices: Vec<u32>,
    ) -> Self {
        let name = name.into();
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{} Vertex Buffer", name)),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{} Index Buffer", name)),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            vertex_buffer,
            index_buffer,
            num_indices: indices.len() as u32,
            name,
        }
    }

    /// Create a sphere model
    pub fn sphere(device: &wgpu::Device, rings: u32, sectors: u32) -> Self {
        let (vertices, indices) = Self::create_sphere(rings, sectors);
        Self::new(device, "Sphere", vertices, indices)
    }

    /// Create a cube model
    pub fn cube(device: &wgpu::Device) -> Self {
        let (vertices, indices) = Self::create_cube();
        Self::new(device, "Cube", vertices, indices)
    }

    /// Render the model
    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }

    /// Set vertex and index buffers without drawing (useful for instanced rendering)
    pub fn set_buffers<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
    }

    fn create_sphere(rings: u32, sectors: u32) -> (Vec<PositionNormal>, Vec<u32>) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let r = 1.0;
        let sector_step = 2.0 * std::f32::consts::PI / sectors as f32;
        let stack_step = std::f32::consts::PI / rings as f32;

        for i in 0..=rings {
            let stack_angle = std::f32::consts::PI / 2.0 - i as f32 * stack_step;
            let xy = r * stack_angle.cos();
            let z = r * stack_angle.sin();

            for j in 0..=sectors {
                let sector_angle = j as f32 * sector_step;
                let x = xy * sector_angle.cos();
                let y = xy * sector_angle.sin();

                vertices.push(PositionNormal::new(
                    [x, y, z],
                    [x, y, z], // For a unit sphere, normal = position
                ));
            }
        }

        for i in 0..rings {
            let mut k1 = i * (sectors + 1);
            let mut k2 = k1 + sectors + 1;

            for _j in 0..sectors {
                if i != 0 {
                    indices.extend_from_slice(&[k1, k2, k1 + 1]);
                }
                if i != (rings - 1) {
                    indices.extend_from_slice(&[k1 + 1, k2, k2 + 1]);
                }
                k1 += 1;
                k2 += 1;
            }
        }

        (vertices, indices)
    }

    fn create_cube() -> (Vec<PositionNormal>, Vec<u32>) {
        let vertices = vec![
            // Front face
            PositionNormal::new([-0.5, -0.5,  0.5], [0.0, 0.0, 1.0]),
            PositionNormal::new([ 0.5, -0.5,  0.5], [0.0, 0.0, 1.0]),
            PositionNormal::new([ 0.5,  0.5,  0.5], [0.0, 0.0, 1.0]),
            PositionNormal::new([-0.5,  0.5,  0.5], [0.0, 0.0, 1.0]),
            // Back face
            PositionNormal::new([-0.5, -0.5, -0.5], [0.0, 0.0, -1.0]),
            PositionNormal::new([-0.5,  0.5, -0.5], [0.0, 0.0, -1.0]),
            PositionNormal::new([ 0.5,  0.5, -0.5], [0.0, 0.0, -1.0]),
            PositionNormal::new([ 0.5, -0.5, -0.5], [0.0, 0.0, -1.0]),
            // Top face
            PositionNormal::new([-0.5,  0.5, -0.5], [0.0, 1.0, 0.0]),
            PositionNormal::new([-0.5,  0.5,  0.5], [0.0, 1.0, 0.0]),
            PositionNormal::new([ 0.5,  0.5,  0.5], [0.0, 1.0, 0.0]),
            PositionNormal::new([ 0.5,  0.5, -0.5], [0.0, 1.0, 0.0]),
            // Bottom face
            PositionNormal::new([-0.5, -0.5, -0.5], [0.0, -1.0, 0.0]),
            PositionNormal::new([ 0.5, -0.5, -0.5], [0.0, -1.0, 0.0]),
            PositionNormal::new([ 0.5, -0.5,  0.5], [0.0, -1.0, 0.0]),
            PositionNormal::new([-0.5, -0.5,  0.5], [0.0, -1.0, 0.0]),
            // Right face
            PositionNormal::new([ 0.5, -0.5, -0.5], [1.0, 0.0, 0.0]),
            PositionNormal::new([ 0.5,  0.5, -0.5], [1.0, 0.0, 0.0]),
            PositionNormal::new([ 0.5,  0.5,  0.5], [1.0, 0.0, 0.0]),
            PositionNormal::new([ 0.5, -0.5,  0.5], [1.0, 0.0, 0.0]),
            // Left face
            PositionNormal::new([-0.5, -0.5, -0.5], [-1.0, 0.0, 0.0]),
            PositionNormal::new([-0.5, -0.5,  0.5], [-1.0, 0.0, 0.0]),
            PositionNormal::new([-0.5,  0.5,  0.5], [-1.0, 0.0, 0.0]),
            PositionNormal::new([-0.5,  0.5, -0.5], [-1.0, 0.0, 0.0]),
        ];

        let indices = vec![
            0,  1,  2,  2,  3,  0,  // front
            4,  5,  6,  6,  7,  4,  // back
            8,  9,  10, 10, 11, 8,  // top
            12, 13, 14, 14, 15, 12, // bottom
            16, 17, 18, 18, 19, 16, // right
            20, 21, 22, 22, 23, 20, // left
        ];

        (vertices, indices)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_cube() {
        let (vertices, indices) = Model::create_cube();
        assert_eq!(vertices.len(), 24); // 6 faces * 4 vertices
        assert_eq!(indices.len(), 36); // 6 faces * 2 triangles * 3 vertices
    }

    #[test]
    fn test_create_sphere() {
        let (vertices, indices) = Model::create_sphere(16, 16);
        assert_eq!(vertices.len(), (16 + 1) * (16 + 1));
        assert!(indices.len() > 0);
    }
}
