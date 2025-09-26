//! Graphics-related ECS systems

use specs::{System, ReadExpect};
// use rustforge_core::prelude::*;
use rustforge_ecs::prelude::*;
use crate::components::*;
use crate::Renderer;

/// Render system that draws all renderable entities
pub struct RenderSystem;

impl<'a> System<'a> for RenderSystem {
    type SystemData = (
        ReadExpect<'a, Renderer>,
        ReadStorage<'a, TransformComponent>,
        ReadStorage<'a, Renderable>,
        ReadStorage<'a, Active>,
    );

    fn run(&mut self, (_renderer, _transforms, _renderables, _actives): Self::SystemData) {
        // TODO: Implement actual rendering
        // This will query renderable entities and submit draw calls
    }
}
