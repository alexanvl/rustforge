//! Spatial query structures and algorithms

use glam::{Mat4, Vec3, Vec4};

pub use crate::node::BoundingBox;

/// View frustum for culling
#[derive(Debug, Clone)]
pub struct Frustum {
    /// Frustum planes in world space (normal.xyz, distance)
    pub planes: [Vec4; 6],
}

impl Frustum {
    pub const PLANE_NEAR: usize = 0;
    pub const PLANE_FAR: usize = 1;
    pub const PLANE_LEFT: usize = 2;
    pub const PLANE_RIGHT: usize = 3;
    pub const PLANE_TOP: usize = 4;
    pub const PLANE_BOTTOM: usize = 5;

    /// Create frustum from view-projection matrix
    pub fn from_matrix(view_proj: &Mat4) -> Self {
        let mut planes = [Vec4::ZERO; 6];

        // Extract frustum planes using Gribb-Hartmann method
        let m = view_proj.to_cols_array();

        // Left plane
        planes[Self::PLANE_LEFT] = Vec4::new(m[3] + m[0], m[7] + m[4], m[11] + m[8], m[15] + m[12]);

        // Right plane
        planes[Self::PLANE_RIGHT] =
            Vec4::new(m[3] - m[0], m[7] - m[4], m[11] - m[8], m[15] - m[12]);

        // Top plane
        planes[Self::PLANE_TOP] = Vec4::new(m[3] - m[1], m[7] - m[5], m[11] - m[9], m[15] - m[13]);

        // Bottom plane
        planes[Self::PLANE_BOTTOM] =
            Vec4::new(m[3] + m[1], m[7] + m[5], m[11] + m[9], m[15] + m[13]);

        // Near plane
        planes[Self::PLANE_NEAR] =
            Vec4::new(m[3] + m[2], m[7] + m[6], m[11] + m[10], m[15] + m[14]);

        // Far plane
        planes[Self::PLANE_FAR] = Vec4::new(m[3] - m[2], m[7] - m[6], m[11] - m[10], m[15] - m[14]);

        // Normalize planes
        for plane in &mut planes {
            let length = plane.truncate().length();
            if length > 0.0 {
                *plane /= length;
            }
        }

        Self { planes }
    }

    /// Test if a point is inside the frustum
    pub fn contains_point(&self, point: Vec3) -> bool {
        for plane in &self.planes {
            let distance = plane.truncate().dot(point) + plane.w;
            if distance < 0.0 {
                return false;
            }
        }
        true
    }

    /// Test if a bounding box intersects the frustum
    pub fn intersects_box(&self, bbox: &BoundingBox) -> bool {
        for plane in &self.planes {
            let normal = plane.truncate();

            // Find the vertex furthest in the direction of the plane normal
            let positive = Vec3::new(
                if normal.x > 0.0 {
                    bbox.max.x
                } else {
                    bbox.min.x
                },
                if normal.y > 0.0 {
                    bbox.max.y
                } else {
                    bbox.min.y
                },
                if normal.z > 0.0 {
                    bbox.max.z
                } else {
                    bbox.min.z
                },
            );

            if normal.dot(positive) + plane.w < 0.0 {
                return false;
            }
        }
        true
    }
}

/// Ray for raycasting
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }

    /// Get a point along the ray
    pub fn point_at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }

    /// Test intersection with a bounding box
    pub fn intersects_box(&self, bbox: &BoundingBox) -> Option<f32> {
        let inv_dir = Vec3::ONE / self.direction;

        let t1 = (bbox.min - self.origin) * inv_dir;
        let t2 = (bbox.max - self.origin) * inv_dir;

        let t_min = t1.min(t2);
        let t_max = t1.max(t2);

        let t_enter = t_min.x.max(t_min.y).max(t_min.z);
        let t_exit = t_max.x.min(t_max.y).min(t_max.z);

        if t_enter <= t_exit && t_exit >= 0.0 {
            Some(t_enter.max(0.0))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::Mat4;

    #[test]
    fn test_frustum_planes_extracted() {
        // Test that frustum planes are extracted correctly
        let proj = Mat4::orthographic_rh(-1.0, 1.0, -1.0, 1.0, 0.1, 10.0);
        let frustum = Frustum::from_matrix(&proj);

        // Should have 6 planes
        assert_eq!(frustum.planes.len(), 6);

        // All planes should be normalized (length ~= 1)
        for plane in &frustum.planes {
            let normal_length = plane.truncate().length();
            assert!((normal_length - 1.0).abs() < 0.01 || normal_length == 0.0);
        }
    }

    #[test]
    fn test_frustum_box_intersection() {
        // Simple perspective frustum looking down -Z
        let proj = Mat4::perspective_rh(std::f32::consts::FRAC_PI_2, 1.0, 0.1, 100.0);
        let view = Mat4::look_at_rh(Vec3::new(0.0, 0.0, 5.0), Vec3::new(0.0, 0.0, 0.0), Vec3::Y);
        let frustum = Frustum::from_matrix(&(proj * view));

        // Box at origin should be visible
        let center_box = BoundingBox::new(Vec3::splat(-1.0), Vec3::splat(1.0));
        assert!(frustum.intersects_box(&center_box));

        // Box far to the side should not be visible
        let side_box = BoundingBox::new(Vec3::new(50.0, 0.0, 0.0), Vec3::new(51.0, 1.0, 1.0));
        assert!(!frustum.intersects_box(&side_box));
    }

    #[test]
    fn test_ray_creation() {
        let ray = Ray::new(Vec3::ZERO, Vec3::new(1.0, 2.0, 0.0));
        assert_eq!(ray.origin, Vec3::ZERO);

        // Direction should be normalized
        let expected_dir = Vec3::new(1.0, 2.0, 0.0).normalize();
        assert!((ray.direction - expected_dir).length() < 0.001);
    }

    #[test]
    fn test_ray_box_intersection() {
        let ray = Ray::new(Vec3::new(-5.0, 0.0, 0.0), Vec3::X);
        let bbox = BoundingBox::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::ONE);

        let t = ray.intersects_box(&bbox);
        assert!(t.is_some());
        assert!((t.unwrap() - 4.0).abs() < 0.001);

        // Ray pointing away
        let ray2 = Ray::new(Vec3::new(-5.0, 0.0, 0.0), Vec3::NEG_X);
        assert!(ray2.intersects_box(&bbox).is_none());

        // Ray missing the box
        let ray3 = Ray::new(Vec3::new(-5.0, 5.0, 0.0), Vec3::X);
        assert!(ray3.intersects_box(&bbox).is_none());
    }
}
