//! Error handling for RustForge

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Graphics error: {0}")]
    Graphics(String),

    #[error("Physics error: {0}")]
    Physics(String),

    #[error("ECS error: {0}")]
    Ecs(String),

    #[error("Asset loading error: {0}")]
    Asset(String),

    #[error("Input error: {0}")]
    Input(String),

    #[error("Initialization error: {0}")]
    Init(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Other error: {0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let graphics_err = Error::Graphics("Vulkan initialization failed".to_string());
        assert!(format!("{}", graphics_err).contains("Graphics error"));
        assert!(format!("{}", graphics_err).contains("Vulkan initialization failed"));

        let physics_err = Error::Physics("Rigid body creation failed".to_string());
        assert!(format!("{}", physics_err).contains("Physics error"));

        let init_err = Error::Init("Window creation failed".to_string());
        assert!(format!("{}", init_err).contains("Initialization error"));
    }

    #[test]
    fn test_error_from_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let rustforge_err: Error = io_err.into();

        match rustforge_err {
            Error::Io(_) => {} // Expected
            _ => panic!("Expected Io error variant"),
        }
    }

    #[test]
    fn test_result_type() {
        fn success_function() -> Result<i32> {
            Ok(42)
        }

        fn error_function() -> Result<i32> {
            Err(Error::Other("Test error".to_string()))
        }

        assert_eq!(success_function().unwrap(), 42);
        assert!(error_function().is_err());
    }
}
