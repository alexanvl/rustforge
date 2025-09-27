//! Physics module using Rapier3D

pub mod collider;
pub mod components;
pub mod rigid_body;
pub mod systems;
pub mod world;

pub use collider::ColliderShape;
pub use rigid_body::RigidBodyType;
pub use world::PhysicsWorld;

/// Re-export physics types
pub mod prelude {
    pub use super::components::*;
    pub use super::systems::*;
    pub use super::{ColliderShape, PhysicsWorld, RigidBodyType};
    pub use rapier3d::prelude::{Point, Real, Rotation, Vector};
}
