//! Material system for PBR rendering

use glam::{Vec3, Vec4};

/// Material properties for rendering
#[derive(Debug, Clone)]
pub struct Material {
    pub name: String,
    pub albedo: Vec4,
    pub metallic: f32,
    pub roughness: f32,
    pub emissive: Vec3,
    pub albedo_texture: Option<String>,
    pub normal_texture: Option<String>,
    pub metallic_roughness_texture: Option<String>,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            name: "Default".into(),
            albedo: Vec4::ONE,
            metallic: 0.0,
            roughness: 0.5,
            emissive: Vec3::ZERO,
            albedo_texture: None,
            normal_texture: None,
            metallic_roughness_texture: None,
        }
    }
}

impl Material {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    /// Create a basic colored material
    pub fn colored(name: impl Into<String>, color: Vec3) -> Self {
        Self {
            name: name.into(),
            albedo: color.extend(1.0),
            ..Default::default()
        }
    }

    /// Create a metallic material
    pub fn metallic(name: impl Into<String>, color: Vec3, metallic: f32, roughness: f32) -> Self {
        Self {
            name: name.into(),
            albedo: color.extend(1.0),
            metallic,
            roughness,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::{Vec3, Vec4};

    #[test]
    fn test_material_default() {
        let material = Material::default();

        assert_eq!(material.name, "Default");
        assert_eq!(material.albedo, Vec4::ONE);
        assert_eq!(material.metallic, 0.0);
        assert_eq!(material.roughness, 0.5);
        assert_eq!(material.emissive, Vec3::ZERO);
        assert!(material.albedo_texture.is_none());
        assert!(material.normal_texture.is_none());
        assert!(material.metallic_roughness_texture.is_none());
    }

    #[test]
    fn test_material_new() {
        let material = Material::new("TestMaterial");

        assert_eq!(material.name, "TestMaterial");
        assert_eq!(material.albedo, Vec4::ONE); // Should inherit from default
        assert_eq!(material.metallic, 0.0);
        assert_eq!(material.roughness, 0.5);
    }

    #[test]
    fn test_material_colored() {
        let color = Vec3::new(1.0, 0.5, 0.0); // Orange
        let material = Material::colored("OrangeMaterial", color);

        assert_eq!(material.name, "OrangeMaterial");
        assert_eq!(material.albedo, color.extend(1.0));
        assert_eq!(material.metallic, 0.0); // Should inherit from default
        assert_eq!(material.roughness, 0.5);
    }

    #[test]
    fn test_material_metallic() {
        let color = Vec3::new(0.8, 0.8, 0.9); // Silver-ish
        let metallic = 0.9;
        let roughness = 0.1;
        let material = Material::metallic("SilverMaterial", color, metallic, roughness);

        assert_eq!(material.name, "SilverMaterial");
        assert_eq!(material.albedo, color.extend(1.0));
        assert_eq!(material.metallic, metallic);
        assert_eq!(material.roughness, roughness);
        assert_eq!(material.emissive, Vec3::ZERO); // Should inherit from default
    }

    #[test]
    fn test_material_properties_range() {
        let material = Material::metallic("Test", Vec3::ONE, 1.0, 1.0);

        // Metallic and roughness should be in valid range [0, 1]
        assert!(material.metallic >= 0.0 && material.metallic <= 1.0);
        assert!(material.roughness >= 0.0 && material.roughness <= 1.0);

        // Albedo should have valid alpha
        assert!(material.albedo.w >= 0.0 && material.albedo.w <= 1.0);
    }

    #[test]
    fn test_material_texture_handling() {
        let mut material = Material::default();

        // Test setting textures
        material.albedo_texture = Some("albedo.png".to_string());
        material.normal_texture = Some("normal.png".to_string());
        material.metallic_roughness_texture = Some("metallic_roughness.png".to_string());

        assert_eq!(material.albedo_texture, Some("albedo.png".to_string()));
        assert_eq!(material.normal_texture, Some("normal.png".to_string()));
        assert_eq!(material.metallic_roughness_texture, Some("metallic_roughness.png".to_string()));

        // Test clearing textures
        material.albedo_texture = None;
        assert!(material.albedo_texture.is_none());
    }
}
