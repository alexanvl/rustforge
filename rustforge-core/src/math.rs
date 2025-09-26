//! Math utilities and extensions for RustForge

use glam::{Vec3, Quat, Mat4};

/// Transform component for position, rotation, and scale
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }
}

impl Transform {
    pub fn new(position: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self { position, rotation, scale }
    }

    /// Create transform matrix
    pub fn matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }

    /// Get forward direction
    pub fn forward(&self) -> Vec3 {
        self.rotation * Vec3::NEG_Z
    }

    /// Get right direction
    pub fn right(&self) -> Vec3 {
        self.rotation * Vec3::X
    }

    /// Get up direction
    pub fn up(&self) -> Vec3 {
        self.rotation * Vec3::Y
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::{Vec3, Quat};

    #[test]
    fn test_transform_default() {
        let t = Transform::default();
        assert_eq!(t.position, Vec3::ZERO);
        assert_eq!(t.rotation, Quat::IDENTITY);
        assert_eq!(t.scale, Vec3::ONE);
    }

    #[test]
    fn test_transform_new() {
        let pos = Vec3::new(1.0, 2.0, 3.0);
        let rot = Quat::from_rotation_y(std::f32::consts::PI / 2.0);
        let scale = Vec3::new(2.0, 2.0, 2.0);

        let t = Transform::new(pos, rot, scale);
        assert_eq!(t.position, pos);
        assert_eq!(t.rotation, rot);
        assert_eq!(t.scale, scale);
    }

    #[test]
    fn test_transform_matrix() {
        let t = Transform::default();
        let matrix = t.matrix();
        assert_eq!(matrix, Mat4::IDENTITY);

        let t2 = Transform::new(
            Vec3::new(1.0, 0.0, 0.0),
            Quat::IDENTITY,
            Vec3::ONE
        );
        let matrix2 = t2.matrix();
        let expected = Mat4::from_translation(Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(matrix2, expected);
    }

    #[test]
    fn test_transform_directions() {
        let t = Transform::default();

        // Default transform should have standard directions
        assert_eq!(t.forward(), Vec3::NEG_Z);
        assert_eq!(t.right(), Vec3::X);
        assert_eq!(t.up(), Vec3::Y);

        // Rotated transform
        let t2 = Transform::new(
            Vec3::ZERO,
            Quat::from_rotation_y(std::f32::consts::PI / 2.0), // 90 degrees
            Vec3::ONE
        );

        // After 90 degree Y rotation, forward should point left (-X)
        // (rotating around Y axis by 90 degrees rotates -Z to -X)
        assert!((t2.forward() - Vec3::NEG_X).length() < 0.001);
        assert!((t2.right() - Vec3::NEG_Z).length() < 0.001);
        assert!((t2.up() - Vec3::Y).length() < 0.001); // Up should remain unchanged
    }
}
