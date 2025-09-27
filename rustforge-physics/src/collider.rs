//! Collider shapes and creation

use glam::Vec3;
use rapier3d::prelude::*;

/// Collider shape types
#[derive(Debug, Clone)]
pub enum ColliderShape {
    Box {
        half_extents: Vec3,
    },
    Sphere {
        radius: f32,
    },
    Capsule {
        half_height: f32,
        radius: f32,
    },
    Cylinder {
        half_height: f32,
        radius: f32,
    },
    Cone {
        half_height: f32,
        radius: f32,
    },
    ConvexMesh {
        vertices: Vec<Vec3>,
    },
    TriMesh {
        vertices: Vec<Vec3>,
        indices: Vec<[u32; 3]>,
    },
}

impl ColliderShape {
    /// Create collider from shape
    pub fn build_collider(&self) -> ColliderBuilder {
        match self {
            ColliderShape::Box { half_extents } => {
                ColliderBuilder::cuboid(half_extents.x, half_extents.y, half_extents.z)
            }
            ColliderShape::Sphere { radius } => ColliderBuilder::ball(*radius),
            ColliderShape::Capsule {
                half_height,
                radius,
            } => ColliderBuilder::capsule_y(*half_height, *radius),
            ColliderShape::Cylinder {
                half_height,
                radius,
            } => ColliderBuilder::cylinder(*half_height, *radius),
            ColliderShape::Cone {
                half_height,
                radius,
            } => ColliderBuilder::cone(*half_height, *radius),
            ColliderShape::ConvexMesh { vertices } => {
                let points: Vec<Point<f32>> =
                    vertices.iter().map(|v| point![v.x, v.y, v.z]).collect();
                ColliderBuilder::convex_hull(&points).unwrap()
            }
            ColliderShape::TriMesh { vertices, indices } => {
                let verts: Vec<Point<f32>> =
                    vertices.iter().map(|v| point![v.x, v.y, v.z]).collect();
                ColliderBuilder::trimesh(verts, indices.clone())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_collider() {
        let shape = ColliderShape::Box {
            half_extents: Vec3::new(1.0, 2.0, 3.0),
        };
        let collider = shape.build_collider().build();

        // Verify collider was created (can't easily test internal shape)
        assert_eq!(collider.friction(), 0.5); // Default friction
        assert_eq!(collider.restitution(), 0.0); // Default restitution
    }

    #[test]
    fn test_sphere_collider() {
        let shape = ColliderShape::Sphere { radius: 2.5 };
        let collider = shape
            .build_collider()
            .friction(0.8)
            .restitution(0.3)
            .build();

        assert_eq!(collider.friction(), 0.8);
        assert_eq!(collider.restitution(), 0.3);
    }

    #[test]
    fn test_capsule_collider() {
        let shape = ColliderShape::Capsule {
            half_height: 1.0,
            radius: 0.5,
        };
        let collider = shape.build_collider().sensor(true).build();

        assert!(collider.is_sensor());
    }

    #[test]
    fn test_cylinder_collider() {
        let shape = ColliderShape::Cylinder {
            half_height: 2.0,
            radius: 1.0,
        };
        let collider = shape
            .build_collider()
            .friction(0.7)
            .restitution(0.2)
            .build();

        assert_eq!(collider.friction(), 0.7);
        assert_eq!(collider.restitution(), 0.2);
    }

    #[test]
    fn test_cone_collider() {
        let shape = ColliderShape::Cone {
            half_height: 1.5,
            radius: 0.8,
        };
        let collider = shape.build_collider().build();

        // Basic sanity check - collider should be created
        assert_eq!(collider.friction(), 0.5); // Default friction
    }

    #[test]
    fn test_convex_mesh_collider() {
        let vertices = vec![
            Vec3::new(-1.0, -1.0, -1.0),
            Vec3::new(1.0, -1.0, -1.0),
            Vec3::new(1.0, 1.0, -1.0),
            Vec3::new(-1.0, 1.0, -1.0),
            Vec3::new(-1.0, -1.0, 1.0),
            Vec3::new(1.0, -1.0, 1.0),
            Vec3::new(1.0, 1.0, 1.0),
            Vec3::new(-1.0, 1.0, 1.0),
        ];

        let shape = ColliderShape::ConvexMesh { vertices };
        let collider = shape.build_collider().build();

        // Basic sanity check
        assert_eq!(collider.friction(), 0.5);
    }

    #[test]
    fn test_trimesh_collider() {
        let vertices = vec![
            Vec3::new(-1.0, -1.0, 0.0),
            Vec3::new(1.0, -1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ];
        let indices = vec![[0, 1, 2]];

        let shape = ColliderShape::TriMesh { vertices, indices };
        let collider = shape.build_collider().build();

        // Basic sanity check
        assert_eq!(collider.friction(), 0.5);
    }

    #[test]
    fn test_collider_properties() {
        let shape = ColliderShape::Sphere { radius: 1.0 };
        let collider = shape
            .build_collider()
            .friction(0.8)
            .restitution(0.3)
            .sensor(false)
            .density(2.0)
            .build();

        assert_eq!(collider.friction(), 0.8);
        assert_eq!(collider.restitution(), 0.3);
        assert!(!collider.is_sensor());
        assert_eq!(collider.density(), 2.0);
    }
}
