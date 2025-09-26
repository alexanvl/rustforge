//! Physics world management using Rapier

use rapier3d::prelude::*;
use rustforge_core::prelude::*;
use std::collections::HashMap;

/// Physics world wrapper around Rapier
pub struct PhysicsWorld {
    pub gravity: Vector<f32>,
    pub integration_parameters: IntegrationParameters,
    pub physics_pipeline: PhysicsPipeline,
    pub island_manager: IslandManager,
    pub broad_phase: BroadPhase,
    pub narrow_phase: NarrowPhase,
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    pub ccd_solver: CCDSolver,
    pub query_pipeline: QueryPipeline,

    // Track entity to physics handle mappings
    rigid_body_map: HashMap<specs::Entity, RigidBodyHandle>,
    collider_map: HashMap<specs::Entity, ColliderHandle>,
}

impl PhysicsWorld {
    /// Create new physics world
    pub fn new(gravity: Vec3) -> Self {
        let mut integration_parameters = IntegrationParameters::default();
        integration_parameters.dt = 1.0 / 60.0; // Fixed 60Hz timestep

        Self {
            gravity: vector![gravity.x, gravity.y, gravity.z],
            integration_parameters,
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            query_pipeline: QueryPipeline::new(),
            rigid_body_map: HashMap::new(),
            collider_map: HashMap::new(),
        }
    }

    /// Step physics simulation
    pub fn step(&mut self) {
        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            None,
            &(),
            &(),
        );

        // Update query pipeline
        self.query_pipeline.update(&self.rigid_body_set, &self.collider_set);
    }

    /// Add rigid body to world
    pub fn add_rigid_body(&mut self, entity: specs::Entity, body: RigidBody) -> RigidBodyHandle {
        let handle = self.rigid_body_set.insert(body);
        self.rigid_body_map.insert(entity, handle);
        handle
    }

    /// Add collider to world
    pub fn add_collider(
        &mut self,
        entity: specs::Entity,
        collider: Collider,
        parent: RigidBodyHandle,
    ) -> ColliderHandle {
        let handle = self.collider_set.insert_with_parent(collider, parent, &mut self.rigid_body_set);
        self.collider_map.insert(entity, handle);
        handle
    }

    /// Get rigid body handle for entity
    pub fn get_rigid_body_handle(&self, entity: specs::Entity) -> Option<RigidBodyHandle> {
        self.rigid_body_map.get(&entity).copied()
    }

    /// Get rigid body
    pub fn get_rigid_body(&self, handle: RigidBodyHandle) -> Option<&RigidBody> {
        self.rigid_body_set.get(handle)
    }

    /// Get mutable rigid body
    pub fn get_rigid_body_mut(&mut self, handle: RigidBodyHandle) -> Option<&mut RigidBody> {
        self.rigid_body_set.get_mut(handle)
    }
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        Self::new(Vec3::new(0.0, -9.81, 0.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rigid_body::create_rigid_body;
    use crate::RigidBodyType;
    use specs::{WorldExt, Builder};

    #[test]
    fn test_physics_world_creation() {
        let world = PhysicsWorld::new(Vec3::new(0.0, -9.81, 0.0));
        assert_eq!(world.gravity.x, 0.0);
        assert_eq!(world.gravity.y, -9.81);
        assert_eq!(world.gravity.z, 0.0);
        assert_eq!(world.integration_parameters.dt, 1.0 / 60.0);
    }

    #[test]
    fn test_default_physics_world() {
        let world = PhysicsWorld::default();
        assert_eq!(world.gravity.y, -9.81);
    }

    #[test]
    fn test_add_rigid_body() {
        let mut world = PhysicsWorld::new(Vec3::new(0.0, -9.81, 0.0));
        // Create a dummy entity for testing
        let mut world_specs = specs::World::new();
        let entity = world_specs.create_entity().build();

        let body = create_rigid_body(
            RigidBodyType::Dynamic,
            Vec3::new(1.0, 2.0, 3.0),
            glam::Quat::IDENTITY
        ).build();

        let handle = world.add_rigid_body(entity, body);

        // Verify body was added
        assert_eq!(world.rigid_body_set.len(), 1);
        assert_eq!(world.get_rigid_body_handle(entity), Some(handle));
        assert!(world.get_rigid_body(handle).is_some());
    }

    #[test]
    fn test_add_collider() {
        let mut world = PhysicsWorld::new(Vec3::new(0.0, -9.81, 0.0));
        // Create a dummy entity for testing
        let mut world_specs = specs::World::new();
        let entity = world_specs.create_entity().build();

        // First add a rigid body
        let body = create_rigid_body(
            RigidBodyType::Dynamic,
            Vec3::ZERO,
            glam::Quat::IDENTITY
        ).build();
        let body_handle = world.add_rigid_body(entity, body);

        // Then add a collider
        let collider = ColliderBuilder::ball(1.0).build();
        let collider_handle = world.add_collider(entity, collider, body_handle);

        // Verify collider was added
        assert_eq!(world.collider_set.len(), 1);
        assert_eq!(world.collider_map.get(&entity), Some(&collider_handle));
    }

    #[test]
    fn test_physics_step() {
        let mut world = PhysicsWorld::new(Vec3::new(0.0, -9.81, 0.0));
        // Create a dummy entity for testing
        let mut world_specs = specs::World::new();
        let entity = world_specs.create_entity().build();

        // Add a dynamic body
        let body = create_rigid_body(
            RigidBodyType::Dynamic,
            Vec3::new(0.0, 10.0, 0.0),
            glam::Quat::IDENTITY
        ).build();
        let handle = world.add_rigid_body(entity, body);

        // Add a collider to give the body mass
        let collider = ColliderBuilder::ball(1.0)
            .density(1.0)
            .build();
        world.add_collider(entity, collider, handle);

        // Get initial position
        let initial_y = world.get_rigid_body(handle).unwrap().translation().y;

        // Step physics
        world.step();

        // Position should have changed due to gravity
        let after_y = world.get_rigid_body(handle).unwrap().translation().y;
        assert!(after_y < initial_y, "Body should have fallen due to gravity. Initial: {}, After: {}", initial_y, after_y);
    }
}
