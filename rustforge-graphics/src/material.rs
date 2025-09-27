//! Material system for PBR rendering

use glam::{Vec3, Vec4};

/// Material type enumeration
#[derive(Debug, Clone)]
pub enum MaterialType {
    /// Standard PBR material
    Pbr,
    /// Cube map material for skyboxes
    Cubemap,
}

/// Material properties for rendering
#[derive(Debug, Clone)]
pub struct Material {
    pub name: String,
    pub material_type: MaterialType,
    pub albedo: Vec4,
    pub metallic: f32,
    pub roughness: f32,
    pub emissive: Vec3,
    pub albedo_texture: Option<String>,
    pub normal_texture: Option<String>,
    pub metallic_roughness_texture: Option<String>,
    /// For cubemap materials - array of 6 face textures
    pub cubemap_faces: Option<[String; 6]>,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            name: "Default".into(),
            material_type: MaterialType::Pbr,
            albedo: Vec4::ONE,
            metallic: 0.0,
            roughness: 0.5,
            emissive: Vec3::ZERO,
            albedo_texture: None,
            normal_texture: None,
            metallic_roughness_texture: None,
            cubemap_faces: None,
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

    /// Create a cubemap material for skyboxes
    pub fn cubemap(name: impl Into<String>, faces: [String; 6]) -> Self {
        Self {
            name: name.into(),
            material_type: MaterialType::Cubemap,
            cubemap_faces: Some(faces),
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
        assert!(matches!(material.material_type, MaterialType::Pbr));
        assert_eq!(material.albedo, Vec4::ONE);
        assert_eq!(material.metallic, 0.0);
        assert_eq!(material.roughness, 0.5);
        assert_eq!(material.emissive, Vec3::ZERO);
        assert!(material.albedo_texture.is_none());
        assert!(material.normal_texture.is_none());
        assert!(material.metallic_roughness_texture.is_none());
        assert!(material.cubemap_faces.is_none());
    }

    #[test]
    fn test_material_new() {
        let material = Material::new("TestMaterial");

        assert_eq!(material.name, "TestMaterial");
        assert!(matches!(material.material_type, MaterialType::Pbr));
        assert_eq!(material.albedo, Vec4::ONE); // Should inherit from default
        assert_eq!(material.metallic, 0.0);
        assert_eq!(material.roughness, 0.5);
    }

    #[test]
    fn test_material_colored() {
        let color = Vec3::new(1.0, 0.5, 0.0); // Orange
        let material = Material::colored("OrangeMaterial", color);

        assert_eq!(material.name, "OrangeMaterial");
        assert!(matches!(material.material_type, MaterialType::Pbr));
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
        assert!(matches!(material.material_type, MaterialType::Pbr));
        assert_eq!(material.albedo, color.extend(1.0));
        assert_eq!(material.metallic, metallic);
        assert_eq!(material.roughness, roughness);
        assert_eq!(material.emissive, Vec3::ZERO); // Should inherit from default
    }

    #[test]
    fn test_material_cubemap() {
        let faces = [
            "right.jpg".to_string(),
            "left.jpg".to_string(),
            "top.jpg".to_string(),
            "bottom.jpg".to_string(),
            "front.jpg".to_string(),
            "back.jpg".to_string(),
        ];
        let material = Material::cubemap("SkyboxMaterial", faces.clone());

        assert_eq!(material.name, "SkyboxMaterial");
        assert!(matches!(material.material_type, MaterialType::Cubemap));
        assert_eq!(material.cubemap_faces, Some(faces));
        // Other properties should be default
        assert_eq!(material.albedo, Vec4::ONE);
        assert_eq!(material.metallic, 0.0);
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
        assert_eq!(
            material.metallic_roughness_texture,
            Some("metallic_roughness.png".to_string())
        );

        // Test clearing textures
        material.albedo_texture = None;
        assert!(material.albedo_texture.is_none());
    }
}
