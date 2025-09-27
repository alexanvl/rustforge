//! Application trait and configuration

use crate::context::{Context, RenderContext};
use rustforge_core::prelude::*;
pub use winit::event::{ElementState, WindowEvent};
pub use winit::keyboard::{KeyCode, PhysicalKey};

/// Configuration for creating an application
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub vsync: bool,
    pub resizable: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            title: "RustForge Application".to_string(),
            width: 1024,
            height: 768,
            vsync: true,
            resizable: true,
        }
    }
}

/// Main application trait that users implement
pub trait App: Sized {
    /// Called once during initialization
    fn init(ctx: &mut Context) -> Result<Self>;

    /// Called every frame for updates
    fn update(&mut self, ctx: &mut Context, dt: f32);

    /// Called every frame for rendering
    fn render(&mut self, ctx: &mut RenderContext) -> Result<()>;

    /// Handle window events. Return true if event was handled.
    fn handle_event(&mut self, _ctx: &mut Context, _event: &WindowEvent) -> bool {
        false
    }

    /// Called when window is resized
    fn on_resize(&mut self, _ctx: &mut Context, _width: u32, _height: u32) {
        // Default implementation does nothing
    }

    /// Get application configuration
    fn config() -> AppConfig {
        AppConfig::default()
    }
}
