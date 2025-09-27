//! Vulkan-based graphics module for RustForge

pub mod renderer;
pub mod vulkan;
pub mod pipeline;
pub mod mesh;
pub mod material;
pub mod camera;
pub mod components;
pub mod systems;
pub mod debug_hud;
pub mod text;
pub mod vertex;
pub mod skybox;
pub mod model;

pub use renderer::Renderer;
pub use camera::Camera;

/// Re-export graphics types
pub mod prelude {
    pub use super::renderer::Renderer;
    pub use super::camera::{Camera, CameraType};
    pub use super::mesh::{Mesh, Vertex};
    pub use super::material::{Material, MaterialType};
    pub use super::components::*;
    pub use super::systems::*;
    pub use super::debug_hud::{DebugHud, DebugInfo, FpsCounter};
    pub use super::text::{TextRenderer, TextVertex};
    pub use super::vertex::{Position, PositionNormal, PositionNormalTexcoord};
    pub use super::skybox::Skybox;
    pub use super::model::{Model, ModelUniforms};
}
