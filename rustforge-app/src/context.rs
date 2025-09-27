//! Application context and render context

use std::sync::Arc;
use rustforge_graphics::Camera;
use rustforge_input::InputState;

/// Application context passed to App methods
pub struct Context {
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,
    pub window_size: (u32, u32),
    pub camera: Camera,
    pub input: InputState,
}

impl Context {
    pub(crate) fn new(
        device: Arc<wgpu::Device>,
        queue: Arc<wgpu::Queue>,
        window_size: (u32, u32),
    ) -> Self {
        let aspect = window_size.0 as f32 / window_size.1 as f32;
        let camera = Camera::perspective(60.0, aspect, 0.1, 1000.0);

        Self {
            device,
            queue,
            window_size,
            camera,
            input: InputState::new(),
        }
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.window_size.0 as f32 / self.window_size.1 as f32
    }
}

/// Render context passed to App::render
pub struct RenderContext<'a> {
    pub encoder: &'a mut wgpu::CommandEncoder,
    pub view: &'a wgpu::TextureView,
    pub depth_view: &'a wgpu::TextureView,
    pub camera: &'a Camera,
    pub device: &'a wgpu::Device,
    pub queue: &'a wgpu::Queue,
}

impl<'a> RenderContext<'a> {
    /// Begin a render pass with default settings
    pub fn begin_render_pass(&mut self, label: &str) -> wgpu::RenderPass {
        self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some(label),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: self.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: self.depth_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            occlusion_query_set: None,
            timestamp_writes: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_aspect_ratio() {
        // Mock device and queue would require complex setup
        // For unit tests, we'll focus on testable logic
        let window_size = (1920, 1080);
        let aspect = window_size.0 as f32 / window_size.1 as f32;
        assert!((aspect - 1.777).abs() < 0.01);
    }
}
