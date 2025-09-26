//! ECS World wrapper and utilities

use specs::{World, WorldExt, DispatcherBuilder, Dispatcher};
use rustforge_core::prelude::*;
use crate::components::*;
use crate::systems::*;

/// Wrapper around specs World with RustForge-specific functionality
pub struct EcsWorld {
    world: World,
    dispatcher: Dispatcher<'static, 'static>,
}

impl EcsWorld {
    /// Create a new ECS world with registered components
    pub fn new() -> Result<Self> {
        let mut world = World::new();

        // Register core components
        world.register::<TransformComponent>();
        world.register::<Velocity>();
        world.register::<Name>();
        world.register::<Tag>();
        world.register::<Active>();

        // Insert global resources
        world.insert(Time::new());

        // Build dispatcher with core systems
        let dispatcher = DispatcherBuilder::new()
            .with(VelocitySystem, "velocity", &[])
            .build();

        Ok(Self { world, dispatcher })
    }

    /// Update the ECS world
    pub fn update(&mut self) {
        // Update time
        {
            let mut time = self.world.write_resource::<Time>();
            time.update();
        }

        // Run systems
        self.dispatcher.dispatch(&self.world);
        self.world.maintain();
    }

    /// Get reference to the underlying world
    pub fn world(&self) -> &World {
        &self.world
    }

    /// Get mutable reference to the underlying world
    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    /// Get time resource
    pub fn time(&self) -> Time {
        *self.world.read_resource::<Time>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use specs::Builder;

    #[test]
    fn test_ecs_world_creation() {
        let world = EcsWorld::new();
        assert!(world.is_ok());
    }

    #[test]
    fn test_entity_creation() {
        let mut world = EcsWorld::new().unwrap();

        // Create an entity with components
        let entity = world.world_mut()
            .create_entity()
            .with(TransformComponent::default())
            .with(Name("TestEntity".to_string()))
            .with(Active::new(true))
            .build();

        // Verify entity exists
        assert!(world.world().is_alive(entity));

        // Verify components
        {
            let transforms = world.world().read_storage::<TransformComponent>();
            let names = world.world().read_storage::<Name>();
            let actives = world.world().read_storage::<Active>();

            assert!(transforms.get(entity).is_some());
            assert_eq!(names.get(entity).unwrap().0, "TestEntity");
            assert_eq!(actives.get(entity).unwrap().0, true);
        }
    }

    #[test]
    fn test_time_resource() {
        let world = EcsWorld::new().unwrap();
        let time = world.time();

        assert_eq!(time.delta_seconds(), 0.0);
        assert_eq!(time.elapsed_seconds(), 0.0);
    }

    #[test]
    fn test_multiple_entities() {
        let mut world = EcsWorld::new().unwrap();

        // Create multiple entities
        let entity1 = world.world_mut()
            .create_entity()
            .with(TransformComponent::default())
            .with(Tag("Player".to_string()))
            .build();

        let entity2 = world.world_mut()
            .create_entity()
            .with(TransformComponent::default())
            .with(Tag("Enemy".to_string()))
            .build();

        // Verify both exist
        assert!(world.world().is_alive(entity1));
        assert!(world.world().is_alive(entity2));

        // Verify different tags
        let tags = world.world().read_storage::<Tag>();
        assert_eq!(tags.get(entity1).unwrap().0, "Player");
        assert_eq!(tags.get(entity2).unwrap().0, "Enemy");
    }
}
