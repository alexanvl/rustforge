//! Core utilities and types for RustForge engine

pub mod math;
pub mod time;
pub mod error;

pub use error::{Result, Error};

/// Re-export commonly used types
pub mod prelude {
    pub use crate::math::*;
    pub use crate::time::*;
    pub use crate::error::{Result, Error};
    pub use glam::{Vec2, Vec3, Vec4, Quat, Mat4};
}
