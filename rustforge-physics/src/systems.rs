//! Physics ECS systems

use std::sync::{Arc, Mutex};
use specs::{System, ReadStorage, WriteStorage, Read, Write, Join, Entities, LendJoin};
use rustforge_core::prelude::*;
use rustforge_ecs::prelude::*;
use crate::components::{PhysicsBody, Collider as EcsCollider, Force, Impulse};
use crate::{PhysicsWorld, rigid_body::create_rigid_body};
use rapier3d::prelude::*;

/// System that initializes physics bodies
pub struct PhysicsInitSystem;

impl<'a> System<'a> for PhysicsInitSystem {
    type SystemData = (
        Entities<'a>,
        Write<'a, Arc<Mutex<PhysicsWorld>>>,
        WriteStorage<'a, PhysicsBody>,
        WriteStorage<'a, EcsCollider>,
        ReadStorage<'a, TransformComponent>,
    );

    fn run(&mut self, (entities, physics_world_arc, mut bodies, mut colliders, transforms): Self::SystemData) {
        let mut physics_world = physics_world_arc.lock().unwrap();
        // Initialize rigid bodies
        for (entity, body, transform) in (&entities, &mut bodies, &transforms).join() {
            if body.handle.is_none() {
                let mut rigid_body = create_rigid_body(
                    body.body_type,
                    transform.transform.position,
                    transform.transform.rotation,
                )
                .build();

                // Set additional properties
                rigid_body.set_linear_damping(body.linear_damping);
                rigid_body.set_angular_damping(body.angular_damping);
                // Note: Mass is typically set via colliders, not directly on the body

                let handle = physics_world.add_rigid_body(entity, rigid_body);
                body.handle = Some(handle);
            }
        }

        // Initialize colliders
        for (entity, collider, body) in (&entities, &mut colliders, &bodies).join() {
            if collider.handle.is_none() && body.handle.is_some() {
                let col = collider.shape.build_collider()
                    .friction(collider.friction)
                    .restitution(collider.restitution)
                    .sensor(collider.is_sensor)
                    .build();

                let handle = physics_world.add_collider(entity, col, body.handle.unwrap());
                collider.handle = Some(handle);
            }
        }
    }
}

/// System that applies forces and impulses
pub struct PhysicsForceSystem;

impl<'a> System<'a> for PhysicsForceSystem {
    type SystemData = (
        Write<'a, Arc<Mutex<PhysicsWorld>>>,
        ReadStorage<'a, PhysicsBody>,
        WriteStorage<'a, Force>,
        WriteStorage<'a, Impulse>,
    );

    fn run(&mut self, (physics_world_arc, bodies, mut forces, mut impulses): Self::SystemData) {
        let mut physics_world = physics_world_arc.lock().unwrap();
        // Apply forces
        for (body, force) in (&bodies, &forces).join() {
            if let Some(handle) = body.handle {
                if let Some(rb) = physics_world.get_rigid_body_mut(handle) {
                    rb.add_force(vector![force.force.x, force.force.y, force.force.z], true);
                    rb.add_torque(vector![force.torque.x, force.torque.y, force.torque.z], true);
                }
            }
        }

        // Clear forces after applying
        forces.clear();

        // Apply impulses
        for (body, impulse) in (&bodies, &impulses).join() {
            if let Some(handle) = body.handle {
                if let Some(rb) = physics_world.get_rigid_body_mut(handle) {
                    rb.apply_impulse(vector![impulse.linear.x, impulse.linear.y, impulse.linear.z], true);
                    rb.apply_torque_impulse(vector![impulse.angular.x, impulse.angular.y, impulse.angular.z], true);
                }
            }
        }

        // Clear impulses after applying
        impulses.clear();
    }
}

/// System that syncs transforms from physics to ECS
pub struct PhysicsSyncSystem;

impl<'a> System<'a> for PhysicsSyncSystem {
    type SystemData = (
        Read<'a, Arc<Mutex<PhysicsWorld>>>,
        ReadStorage<'a, PhysicsBody>,
        WriteStorage<'a, TransformComponent>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (physics_world_arc, bodies, mut transforms, mut velocities): Self::SystemData) {
        let physics_world = physics_world_arc.lock().unwrap();
        for (body, transform, vel_opt) in (&bodies, &mut transforms, (&mut velocities).maybe()).join() {
            if let Some(handle) = body.handle {
                if let Some(rb) = physics_world.get_rigid_body(handle) {
                    // Update transform
                    let pos = rb.translation();
                    let rot = rb.rotation();

                    transform.transform.position = Vec3::new(pos.x, pos.y, pos.z);
                    transform.transform.rotation = glam::Quat::from_xyzw(rot.i, rot.j, rot.k, rot.w);

                    // Update velocity if component exists
                    if let Some(velocity) = vel_opt {
                        let lin_vel = rb.linvel();
                        let ang_vel = rb.angvel();

                        velocity.linear = Vec3::new(lin_vel.x, lin_vel.y, lin_vel.z);
                        velocity.angular = Vec3::new(ang_vel.x, ang_vel.y, ang_vel.z);
                    }
                }
            }
        }
    }
}

/// System that steps the physics simulation
pub struct PhysicsStepSystem;

impl<'a> System<'a> for PhysicsStepSystem {
    type SystemData = (
        Write<'a, Arc<Mutex<PhysicsWorld>>>,
        Read<'a, Time>,
    );

    fn run(&mut self, (physics_world_arc, _time): Self::SystemData) {
        let mut physics_world = physics_world_arc.lock().unwrap();
        // Only step if enough time has passed (fixed timestep)
        // The physics world already has its own fixed timestep integration
        physics_world.step();
    }
}
