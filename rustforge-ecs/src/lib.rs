//! Entity-Component-System implementation for RustForge

pub mod components;
pub mod systems;
pub mod world;

pub use specs::{
    Builder, Component, DispatcherBuilder, Entity, Join, ReadStorage, System,
    World, WorldExt, WriteStorage, VecStorage, DenseVecStorage, FlaggedStorage,
};

pub use world::EcsWorld;

/// Re-export common ECS types
pub mod prelude {
    pub use super::{
        Builder, Component, Entity, Join, ReadStorage, System,
        World, WorldExt, WriteStorage, VecStorage, DenseVecStorage,
        EcsWorld,
    };
    pub use super::components::*;
    pub use super::systems::*;
}
