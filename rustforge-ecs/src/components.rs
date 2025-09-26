//! Core ECS components

use specs::{Component, DenseVecStorage, VecStorage, FlaggedStorage};
use specs_derive::Component;
use rustforge_core::prelude::*;

/// Transform component for spatial data
#[derive(Component, Debug, Clone, Copy)]
#[storage(FlaggedStorage)]
pub struct TransformComponent {
    pub transform: Transform,
}

impl Default for TransformComponent {
    fn default() -> Self {
        Self {
            transform: Transform::default(),
        }
    }
}

/// Velocity component for movement
#[derive(Component, Debug, Clone, Copy, Default)]
#[storage(VecStorage)]
pub struct Velocity {
    pub linear: Vec3,
    pub angular: Vec3,
}

/// Name component for entity identification
#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Name(pub String);

/// Tag component for entity categorization
#[derive(Component, Debug, Clone)]
#[storage(DenseVecStorage)]
pub struct Tag(pub String);

/// Active state component
#[derive(Component, Debug, Clone, Copy, Default)]
#[storage(DenseVecStorage)]
pub struct Active(pub bool);

impl Active {
    pub fn new(active: bool) -> Self {
        Self(active)
    }
}

/// Tag component for entities that can be controlled by the player
#[derive(Component, Debug, Default)]
#[storage(DenseVecStorage)]
pub struct PlayerControlled;

#[cfg(test)]
mod tests {
    use super::*;
    use glam::{Vec3, Quat};

    #[test]
    fn test_transform_component_default() {
        let tc = TransformComponent::default();
        assert_eq!(tc.transform.position, Vec3::ZERO);
        assert_eq!(tc.transform.rotation, Quat::IDENTITY);
        assert_eq!(tc.transform.scale, Vec3::ONE);
    }

    #[test]
    fn test_velocity_default() {
        let vel = Velocity::default();
        assert_eq!(vel.linear, Vec3::ZERO);
        assert_eq!(vel.angular, Vec3::ZERO);
    }

    #[test]
    fn test_velocity_creation() {
        let mut vel = Velocity::default();
        vel.linear = Vec3::new(1.0, 2.0, 3.0);
        vel.angular = Vec3::new(0.1, 0.2, 0.3);

        assert_eq!(vel.linear, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(vel.angular, Vec3::new(0.1, 0.2, 0.3));
    }

    #[test]
    fn test_name_component() {
        let name = Name("TestEntity".to_string());
        assert_eq!(name.0, "TestEntity");
    }

    #[test]
    fn test_tag_component() {
        let tag = Tag("Enemy".to_string());
        assert_eq!(tag.0, "Enemy");
    }

    #[test]
    fn test_active_component() {
        let active = Active::new(true);
        assert_eq!(active.0, true);

        let inactive = Active::new(false);
        assert_eq!(inactive.0, false);
    }
}
