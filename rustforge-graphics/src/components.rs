//! Graphics-related ECS components

use glam::Vec3;
use specs::{Component, DenseVecStorage, VecStorage};
use specs_derive::Component;

/// Renderable component marking entities for rendering
#[derive(Component, Debug, Clone)]
#[storage(DenseVecStorage)]
pub struct Renderable {
    pub mesh_name: String,
    pub material_name: String,
}

/// Skybox component for environment mapping
#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Skybox {
    pub cubemap_name: String,
    pub intensity: f32,
}

/// Light component for light sources
#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub enum Light {
    Directional {
        direction: Vec3,
        color: Vec3,
        intensity: f32,
    },
    Point {
        color: Vec3,
        intensity: f32,
        radius: f32,
    },
    Spot {
        direction: Vec3,
        color: Vec3,
        intensity: f32,
        angle: f32,
    },
}

/// Camera marker component
#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct CameraComponent {
    pub active: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_renderable_component() {
        let renderable = Renderable {
            mesh_name: "test_mesh".to_string(),
            material_name: "test_material".to_string(),
        };

        assert_eq!(renderable.mesh_name, "test_mesh");
        assert_eq!(renderable.material_name, "test_material");
    }

    #[test]
    fn test_skybox_component() {
        let skybox = Skybox {
            cubemap_name: "test_cubemap".to_string(),
            intensity: 1.5,
        };

        assert_eq!(skybox.cubemap_name, "test_cubemap");
        assert_eq!(skybox.intensity, 1.5);
    }

    #[test]
    fn test_skybox_default_intensity() {
        let skybox = Skybox {
            cubemap_name: "sky".to_string(),
            intensity: 1.0,
        };

        assert_eq!(skybox.intensity, 1.0);
    }

    #[test]
    fn test_light_variants() {
        let dir_light = Light::Directional {
            direction: Vec3::new(0.0, -1.0, 0.0),
            color: Vec3::new(1.0, 1.0, 1.0),
            intensity: 0.8,
        };

        match dir_light {
            Light::Directional {
                direction,
                color,
                intensity,
            } => {
                assert_eq!(direction, Vec3::new(0.0, -1.0, 0.0));
                assert_eq!(color, Vec3::ONE);
                assert_eq!(intensity, 0.8);
            }
            _ => panic!("Expected directional light"),
        }

        let point_light = Light::Point {
            color: Vec3::new(1.0, 0.8, 0.6),
            intensity: 100.0,
            radius: 10.0,
        };

        match point_light {
            Light::Point {
                color,
                intensity,
                radius,
            } => {
                assert_eq!(color, Vec3::new(1.0, 0.8, 0.6));
                assert_eq!(intensity, 100.0);
                assert_eq!(radius, 10.0);
            }
            _ => panic!("Expected point light"),
        }
    }

    #[test]
    fn test_camera_component() {
        let camera = CameraComponent { active: true };
        assert!(camera.active);

        let default_camera = CameraComponent::default();
        assert!(!default_camera.active);
    }
}
