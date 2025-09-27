//! Entity-Component-System implementation for RustForge

pub mod components;
pub mod systems;
pub mod world;

pub use specs::{
    Builder, Component, DenseVecStorage, DispatcherBuilder, Entity, FlaggedStorage, Join,
    ReadStorage, System, VecStorage, World, WorldExt, WriteStorage,
};

pub use world::EcsWorld;

/// Re-export common ECS types
pub mod prelude {
    pub use super::components::*;
    pub use super::systems::*;
    pub use super::{
        Builder, Component, DenseVecStorage, EcsWorld, Entity, Join, ReadStorage, System,
        VecStorage, World, WorldExt, WriteStorage,
    };
}
