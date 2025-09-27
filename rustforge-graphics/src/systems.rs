//! Graphics-related ECS systems

use specs::{ReadExpect, ReadStorage, System};
// use rustforge_core::prelude::*;
use crate::components::*;
use crate::Renderer;
use rustforge_ecs::prelude::*;

/// Skybox rendering system that renders skyboxes first (behind everything)
pub struct SkyboxRenderSystem;

impl<'a> System<'a> for SkyboxRenderSystem {
    type SystemData = (
        ReadExpect<'a, Renderer>,
        ReadStorage<'a, TransformComponent>,
        ReadStorage<'a, Skybox>,
        ReadStorage<'a, Active>,
    );

    fn run(&mut self, (_renderer, transforms, skyboxes, actives): Self::SystemData) {
        // TODO: Implement skybox rendering
        // This should:
        // 1. Find active skybox entities
        // 2. Load cubemap textures
        // 3. Render skybox cube with cubemap texture
        // 4. Use camera rotation but not translation
        // 5. Render with depth testing disabled or with greater depth

        for (_skybox, transform, active) in (&skyboxes, &transforms, &actives).join() {
            if !active.0 {
                continue;
            }

            // TODO: Implement actual skybox rendering
            // For now, just log that we found a skybox
            println!(
                "Found active skybox at position: {:?}",
                transform.transform.position
            );
        }
    }
}

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
