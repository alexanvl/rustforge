//! Core utilities and types for RustForge engine

pub mod error;
pub mod math;
pub mod time;

pub use error::{Error, Result};

/// Re-export commonly used types
pub mod prelude {
    pub use crate::error::{Error, Result};
    pub use crate::math::*;
    pub use crate::time::*;
    pub use glam::{Mat4, Quat, Vec2, Vec3, Vec4};
}
