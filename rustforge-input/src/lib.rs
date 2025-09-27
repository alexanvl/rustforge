//! Input handling module for RustForge

mod input_state;
mod camera_controller;

pub use input_state::InputState;
pub use camera_controller::CameraController;

/// Re-export input types
pub mod prelude {
    pub use super::InputState;
    pub use super::CameraController;
}
