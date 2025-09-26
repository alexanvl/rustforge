//! Physics module using Rapier3D

pub mod world;
pub mod components;
pub mod systems;
pub mod collider;
pub mod rigid_body;

pub use world::PhysicsWorld;
pub use collider::ColliderShape;
pub use rigid_body::RigidBodyType;

/// Re-export physics types
pub mod prelude {
    pub use super::{
        PhysicsWorld,
        ColliderShape,
        RigidBodyType,
    };
    pub use super::components::*;
    pub use super::systems::*;
    pub use rapier3d::prelude::{Real, Vector, Point, Rotation};
}
