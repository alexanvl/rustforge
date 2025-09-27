//! Core ECS systems

use crate::components::{Active, TransformComponent, Velocity};
use rustforge_core::prelude::*;
use specs::{Join, Read, ReadStorage, System, WriteStorage};

/// System that updates transforms based on velocity
pub struct VelocitySystem;

impl<'a> System<'a> for VelocitySystem {
    type SystemData = (
        WriteStorage<'a, TransformComponent>,
        ReadStorage<'a, Velocity>,
        ReadStorage<'a, Active>,
        Read<'a, Time>,
    );

    fn run(&mut self, (mut transforms, velocities, actives, time): Self::SystemData) {
        let dt = time.delta_seconds();

        for (transform, velocity, _) in (&mut transforms, &velocities, &actives).join() {
            // Update position
            transform.transform.position += velocity.linear * dt;

            // Update rotation
            if velocity.angular.length_squared() > 0.0 {
                let angle = velocity.angular.length() * dt;
                let axis = velocity.angular.normalize();
                let rotation = Quat::from_axis_angle(axis, angle);
                transform.transform.rotation = rotation * transform.transform.rotation;
            }
        }
    }
}
