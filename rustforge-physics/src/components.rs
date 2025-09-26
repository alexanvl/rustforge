//! Physics ECS components

use specs::{Component, VecStorage, DenseVecStorage};
use specs_derive::Component;
use rapier3d::prelude::*;
use glam::Vec3;
use crate::{ColliderShape, RigidBodyType};

/// Physics body component
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct PhysicsBody {
    pub body_type: RigidBodyType,
    pub handle: Option<RigidBodyHandle>,
    pub mass: f32,
    pub linear_damping: f32,
    pub angular_damping: f32,
}

impl PhysicsBody {
    pub fn new(body_type: RigidBodyType) -> Self {
        Self {
            body_type,
            handle: None,
            mass: 1.0,
            linear_damping: 0.0,
            angular_damping: 0.0,
        }
    }
}

/// Collider component
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Collider {
    pub shape: ColliderShape,
    pub handle: Option<ColliderHandle>,
    pub friction: f32,
    pub restitution: f32,
    pub is_sensor: bool,
}

impl Collider {
    pub fn new(shape: ColliderShape) -> Self {
        Self {
            shape,
            handle: None,
            friction: 0.5,
            restitution: 0.0,
            is_sensor: false,
        }
    }
}

/// Force to apply to physics body
#[derive(Component, Debug, Default)]
#[storage(DenseVecStorage)]
pub struct Force {
    pub force: Vec3,
    pub torque: Vec3,
}

/// Impulse to apply to physics body
#[derive(Component, Debug, Default)]
#[storage(DenseVecStorage)]
pub struct Impulse {
    pub linear: Vec3,
    pub angular: Vec3,
}

/// Marks entity as having physics state that needs syncing
#[derive(Component, Debug, Default)]
#[storage(DenseVecStorage)]
pub struct PhysicsSync;

/// Collision event data
#[derive(Debug, Clone)]
pub struct CollisionEvent {
    pub entity_a: specs::Entity,
    pub entity_b: specs::Entity,
    pub normal: Vec3,
    pub depth: f32,
}

/// Component that stores collision events for an entity
#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct CollisionEvents {
    pub events: Vec<CollisionEvent>,
}
