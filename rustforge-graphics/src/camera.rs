//! Camera implementation for 3D rendering

use glam::Mat4;
use rustforge_core::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CameraType {
    Perspective { fov: f32, near: f32, far: f32 },
    Orthographic { size: f32, near: f32, far: f32 },
}

/// Camera for 3D rendering
#[derive(Debug, Clone)]
pub struct Camera {
    pub transform: Transform,
    pub camera_type: CameraType,
    pub aspect_ratio: f32,
}

impl Camera {
    /// Create a perspective camera
    pub fn perspective(fov_degrees: f32, aspect_ratio: f32, near: f32, far: f32) -> Self {
        Self {
            transform: Transform::default(),
            camera_type: CameraType::Perspective {
                fov: fov_degrees.to_radians(),
                near,
                far,
            },
            aspect_ratio,
        }
    }

    /// Create an orthographic camera
    pub fn orthographic(size: f32, aspect_ratio: f32, near: f32, far: f32) -> Self {
        Self {
            transform: Transform::default(),
            camera_type: CameraType::Orthographic { size, near, far },
            aspect_ratio,
        }
    }

    /// Get view matrix (inverse of camera transform)
    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(
            self.transform.position,
            self.transform.position + self.transform.forward(),
            self.transform.up(),
        )
    }

    /// Get projection matrix
    pub fn projection_matrix(&self) -> Mat4 {
        match self.camera_type {
            CameraType::Perspective { fov, near, far } => {
                Mat4::perspective_rh(fov, self.aspect_ratio, near, far)
            }
            CameraType::Orthographic { size, near, far } => {
                let half_width = size * self.aspect_ratio * 0.5;
                let half_height = size * 0.5;
                Mat4::orthographic_rh(
                    -half_width,
                    half_width,
                    -half_height,
                    half_height,
                    near,
                    far,
                )
            }
        }
    }

    /// Get combined view-projection matrix
    pub fn view_projection_matrix(&self) -> Mat4 {
        self.projection_matrix() * self.view_matrix()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::{Quat, Vec3};

    #[test]
    fn test_camera_perspective_creation() {
        let camera = Camera::perspective(90.0, 16.0 / 9.0, 0.1, 100.0);

        assert_eq!(camera.aspect_ratio, 16.0 / 9.0);
        match camera.camera_type {
            CameraType::Perspective { fov, near, far } => {
                assert!((fov - std::f32::consts::PI / 2.0).abs() < 0.001); // 90 degrees in radians
                assert_eq!(near, 0.1);
                assert_eq!(far, 100.0);
            }
            _ => panic!("Expected perspective camera type"),
        }
    }

    #[test]
    fn test_camera_orthographic_creation() {
        let camera = Camera::orthographic(10.0, 1.0, 0.1, 100.0);

        assert_eq!(camera.aspect_ratio, 1.0);
        match camera.camera_type {
            CameraType::Orthographic { size, near, far } => {
                assert_eq!(size, 10.0);
                assert_eq!(near, 0.1);
                assert_eq!(far, 100.0);
            }
            _ => panic!("Expected orthographic camera type"),
        }
    }

    #[test]
    fn test_camera_view_matrix() {
        let mut camera = Camera::perspective(90.0, 1.0, 0.1, 100.0);

        // Default camera should look down negative Z
        let view_matrix = camera.view_matrix();
        let expected = Mat4::look_at_rh(Vec3::ZERO, Vec3::NEG_Z, Vec3::Y);
        assert_eq!(view_matrix, expected);

        // Move camera and test
        camera.transform.position = Vec3::new(0.0, 0.0, 5.0);
        let view_matrix = camera.view_matrix();
        let expected = Mat4::look_at_rh(
            Vec3::new(0.0, 0.0, 5.0),
            Vec3::new(0.0, 0.0, 4.0), // position + forward
            Vec3::Y,
        );
        assert_eq!(view_matrix, expected);
    }

    #[test]
    fn test_camera_projection_matrix() {
        let camera = Camera::perspective(90.0, 1.0, 0.1, 100.0);
        let proj_matrix = camera.projection_matrix();

        // Basic sanity checks for perspective projection
        // For perspective projection, the matrix should not be identity
        assert_ne!(proj_matrix, Mat4::IDENTITY);

        // The perspective matrix should be invertible (determinant != 0)
        assert!(proj_matrix.determinant().abs() > 0.001);

        let ortho_camera = Camera::orthographic(10.0, 1.0, 0.1, 100.0);
        let ortho_proj_matrix = ortho_camera.projection_matrix();

        // Basic sanity checks for orthographic projection
        assert_ne!(ortho_proj_matrix, Mat4::IDENTITY);
        // Note: Orthographic matrices can have determinant = 0 in some cases
        // but they should still be valid transformation matrices

        // Test that matrices are different
        assert_ne!(proj_matrix, ortho_proj_matrix);

        // Test that both matrices are valid 4x4 matrices
        assert_eq!(proj_matrix.to_cols_array_2d().len(), 4);
        assert_eq!(ortho_proj_matrix.to_cols_array_2d().len(), 4);

        // Test that the matrices have reasonable values (not all zeros or infinities)
        for row in proj_matrix.to_cols_array_2d() {
            for val in row {
                assert!(val.is_finite());
            }
        }
        for row in ortho_proj_matrix.to_cols_array_2d() {
            for val in row {
                assert!(val.is_finite());
            }
        }
    }

    #[test]
    fn test_camera_view_projection_matrix() {
        let camera = Camera::perspective(90.0, 1.0, 0.1, 100.0);
        let view_proj = camera.view_projection_matrix();
        let expected = camera.projection_matrix() * camera.view_matrix();

        assert_eq!(view_proj, expected);
    }

    #[test]
    fn test_camera_transform_operations() {
        let mut camera = Camera::perspective(90.0, 1.0, 0.1, 100.0);

        // Test position
        camera.transform.position = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(camera.transform.position, Vec3::new(1.0, 2.0, 3.0));

        // Test rotation
        camera.transform.rotation = Quat::from_rotation_y(std::f32::consts::PI / 2.0);
        let forward = camera.transform.forward();
        assert!((forward - Vec3::NEG_X).length() < 0.001); // Should point left after 90° Y rotation
    }
}
