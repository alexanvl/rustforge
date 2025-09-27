//! Generic engine systems that provide common functionality

use rustforge_core::prelude::*;
use rustforge_ecs::prelude::*;
use specs::{Join, Read, ReadStorage, System, WriteStorage};

/// System that applies velocity to transform components
/// This is a generic system that can be used by any game logic
pub struct VelocitySystem;

impl<'a> System<'a> for VelocitySystem {
    type SystemData = (
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, TransformComponent>,
        ReadStorage<'a, Active>,
        Read<'a, Time>,
    );

    fn run(&mut self, (velocities, mut transforms, actives, time): Self::SystemData) {
        for (velocity, transform, _) in (&velocities, &mut transforms, &actives).join() {
            // Apply linear velocity to position
            transform.transform.position += velocity.linear * time.delta_seconds();

            // Apply angular velocity to rotation
            if velocity.angular.length_squared() > 0.0 {
                let rotation_delta =
                    glam::Quat::from_scaled_axis(velocity.angular * time.delta_seconds());
                transform.transform.rotation = transform.transform.rotation * rotation_delta;
            }
        }
    }
}
