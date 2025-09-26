//! Vulkan-based graphics module for RustForge

pub mod renderer;
pub mod vulkan;
pub mod pipeline;
pub mod mesh;
pub mod material;
pub mod camera;
pub mod components;
pub mod systems;

pub use renderer::Renderer;
pub use camera::Camera;

/// Re-export graphics types
pub mod prelude {
    pub use super::renderer::Renderer;
    pub use super::camera::{Camera, CameraType};
    pub use super::mesh::{Mesh, Vertex};
    pub use super::material::Material;
    pub use super::components::*;
    pub use super::systems::*;
}
