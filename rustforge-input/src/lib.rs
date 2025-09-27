//! Input handling module for RustForge

mod camera_controller;
mod input_state;

pub use camera_controller::CameraController;
pub use input_state::InputState;

/// Re-export input types
pub mod prelude {
    pub use super::CameraController;
    pub use super::InputState;
}
