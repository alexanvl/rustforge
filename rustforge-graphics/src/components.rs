//! Graphics-related ECS components

use specs::{Component, VecStorage, DenseVecStorage};
use specs_derive::Component;
use glam::Vec3;

/// Renderable component marking entities for rendering
#[derive(Component, Debug, Clone)]
#[storage(DenseVecStorage)]
pub struct Renderable {
    pub mesh_name: String,
    pub material_name: String,
}

/// Light component for light sources
#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub enum Light {
    Directional { direction: Vec3, color: Vec3, intensity: f32 },
    Point { color: Vec3, intensity: f32, radius: f32 },
    Spot { direction: Vec3, color: Vec3, intensity: f32, angle: f32 },
}

/// Camera marker component
#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct CameraComponent {
    pub active: bool,
}
