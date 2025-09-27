//! Application framework for RustForge engine
//!
//! This crate provides a high-level application framework that handles:
//! - Window creation and event loop
//! - WGPU initialization and surface management
//! - Input handling
//! - Render loop management
//!
//! This significantly reduces boilerplate for creating RustForge applications.

pub mod app;
pub mod context;
pub mod runner;

#[cfg(test)]
mod simple_demo;

pub mod prelude {
    pub use crate::app::{App, AppConfig, WindowEvent, ElementState, KeyCode, PhysicalKey};
    pub use crate::context::{Context, RenderContext};
    pub use crate::runner::run;

    // Re-export commonly used types
    pub use rustforge_core::prelude::*;
    pub use rustforge_graphics::prelude::*;
    pub use rustforge_input::prelude::*;
    pub use glam::{Vec2, Vec3, Vec4, Mat4, Quat};
}