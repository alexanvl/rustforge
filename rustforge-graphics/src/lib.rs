//! Vulkan-based graphics module for RustForge

pub mod camera;
pub mod components;
pub mod debug_hud;
pub mod material;
pub mod mesh;
pub mod model;
pub mod pipeline;
pub mod renderer;
pub mod skybox;
pub mod systems;
pub mod text;
pub mod vertex;
pub mod vulkan;

pub use camera::Camera;
pub use renderer::Renderer;

/// Re-export graphics types
pub mod prelude {
    pub use super::camera::{Camera, CameraType};
    pub use super::components::*;
    pub use super::debug_hud::{DebugHud, DebugInfo, FpsCounter};
    pub use super::material::{Material, MaterialType};
    pub use super::mesh::{Mesh, Vertex};
    pub use super::model::{Model, ModelUniforms};
    pub use super::renderer::Renderer;
    pub use super::skybox::Skybox;
    pub use super::systems::*;
    pub use super::text::{TextRenderer, TextVertex};
    pub use super::vertex::{Position, PositionNormal, PositionNormalTexcoord};
}
