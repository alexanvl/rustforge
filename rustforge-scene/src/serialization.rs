//! Scene serialization and deserialization

use crate::{Result, SceneGraph, SceneNode};
use glam::{Quat, Vec3};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Root structure for scene files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneFile {
    pub version: String,
    pub nodes: Vec<NodeData>,
    #[serde(default)]
    pub assets: AssetReferences,
}

/// Node data for serialization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeData {
    pub name: String,
    #[serde(default)]
    pub transform: TransformData,
    #[serde(default)]
    pub children: Vec<String>,
    #[serde(default)]
    pub components: Vec<ComponentData>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default = "default_true")]
    pub active: bool,
}

fn default_true() -> bool {
    true
}

/// Transform data for serialization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformData {
    #[serde(default)]
    pub position: [f32; 3],
    #[serde(default = "default_rotation")]
    pub rotation: [f32; 4], // Quaternion as [x, y, z, w]
    #[serde(default = "default_scale")]
    pub scale: [f32; 3],
}

fn default_rotation() -> [f32; 4] {
    [0.0, 0.0, 0.0, 1.0]
}

fn default_scale() -> [f32; 3] {
    [1.0, 1.0, 1.0]
}

impl Default for TransformData {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            rotation: default_rotation(),
            scale: default_scale(),
        }
    }
}

impl From<&rustforge_core::math::Transform> for TransformData {
    fn from(t: &rustforge_core::math::Transform) -> Self {
        Self {
            position: t.position.to_array(),
            rotation: t.rotation.to_array(),
            scale: t.scale.to_array(),
        }
    }
}

impl From<TransformData> for rustforge_core::math::Transform {
    fn from(t: TransformData) -> Self {
        Self {
            position: Vec3::from_array(t.position),
            rotation: Quat::from_array(t.rotation),
            scale: Vec3::from_array(t.scale),
        }
    }
}

/// Component data for serialization
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ComponentData {
    Mesh {
        path: String,
        #[serde(default)]
        cast_shadows: bool,
    },
    Light {
        #[serde(flatten)]
        light_type: LightType,
        color: [f32; 3],
        intensity: f32,
    },
    Camera {
        fov: f32,
        #[serde(default = "default_near")]
        near: f32,
        #[serde(default = "default_far")]
        far: f32,
    },
    RigidBody {
        mass: f32,
        #[serde(default)]
        kinematic: bool,
    },
    Collider {
        #[serde(flatten)]
        shape: ColliderShape,
    },
    Custom {
        component_type: String,
        #[serde(flatten)]
        data: serde_json::Value,
    },
}

fn default_near() -> f32 {
    0.1
}
fn default_far() -> f32 {
    1000.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "light_type")]
pub enum LightType {
    Directional,
    Point {
        range: f32,
    },
    Spot {
        range: f32,
        inner_angle: f32,
        outer_angle: f32,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "shape")]
pub enum ColliderShape {
    Box { size: [f32; 3] },
    Sphere { radius: f32 },
    Capsule { height: f32, radius: f32 },
    Mesh { path: String },
}

/// Asset references used in the scene
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AssetReferences {
    #[serde(default)]
    pub meshes: HashMap<String, String>,
    #[serde(default)]
    pub textures: HashMap<String, String>,
    #[serde(default)]
    pub materials: HashMap<String, String>,
}

impl SceneGraph {
    /// Load scene from JSON file
    pub fn load_json(path: impl AsRef<Path>) -> Result<(Self, SceneFile)> {
        let contents = std::fs::read_to_string(path)?;
        let scene_file: SceneFile = serde_json::from_str(&contents)?;
        let graph = Self::from_scene_file(&scene_file)?;
        Ok((graph, scene_file))
    }

    /// Load scene from RON file
    pub fn load_ron(path: impl AsRef<Path>) -> Result<(Self, SceneFile)> {
        let contents = std::fs::read_to_string(path)?;
        let scene_file: SceneFile = ron::de::from_str(&contents).map_err(ron::Error::from)?;
        let graph = Self::from_scene_file(&scene_file)?;
        Ok((graph, scene_file))
    }

    /// Create scene graph from scene file data
    pub fn from_scene_file(scene_file: &SceneFile) -> Result<Self> {
        let mut graph = SceneGraph::new();
        let mut name_to_id = HashMap::new();

        // First pass: create all nodes
        for node_data in &scene_file.nodes {
            let mut node = SceneNode::new(&node_data.name);
            node.transform = node_data.transform.clone().into();
            node.tags = node_data.tags.iter().cloned().collect();
            node.active = node_data.active;

            let id = graph.add_node(node);
            name_to_id.insert(&node_data.name, id);
        }

        // Second pass: establish hierarchy
        let mut has_parent = std::collections::HashSet::new();

        for node_data in &scene_file.nodes {
            let node_id = name_to_id[&node_data.name];

            for child_name in &node_data.children {
                if let Some(&child_id) = name_to_id.get(child_name) {
                    graph.add_child(node_id, child_id)?;
                    has_parent.insert(child_id);
                }
            }
        }

        // Third pass: add root-level nodes (those without parents) to the scene root
        for &node_id in name_to_id.values() {
            if !has_parent.contains(&node_id) {
                graph.add_child(graph.root(), node_id)?;
            }
        }

        // Update all transforms
        graph.update_transforms();

        Ok(graph)
    }

    /// Save scene to JSON file
    pub fn save_json(&self, path: impl AsRef<Path>) -> Result<()> {
        let scene_file = self.to_scene_file();
        let contents = serde_json::to_string_pretty(&scene_file)?;
        std::fs::write(path, contents)?;
        Ok(())
    }

    /// Save scene to RON file
    pub fn save_ron(&self, path: impl AsRef<Path>) -> Result<()> {
        let scene_file = self.to_scene_file();
        let contents = ron::ser::to_string_pretty(&scene_file, Default::default())?;
        std::fs::write(path, contents)?;
        Ok(())
    }

    /// Convert scene graph to scene file data
    pub fn to_scene_file(&self) -> SceneFile {
        let mut nodes = Vec::new();

        for (id, node) in self.iter() {
            if id == self.root() {
                continue; // Skip root node in serialization
            }

            let children: Vec<String> = node
                .children
                .iter()
                .filter_map(|&child_id| self.get(child_id).ok().map(|n| n.name.clone()))
                .collect();

            let node_data = NodeData {
                name: node.name.clone(),
                transform: (&node.transform).into(),
                children,
                components: Vec::new(), // Components should be handled by ECS
                tags: node.tags.iter().cloned().collect(),
                active: node.active,
            };

            nodes.push(node_data);
        }

        SceneFile {
            version: "1.0".to_string(),
            nodes,
            assets: AssetReferences::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_data_conversion() {
        let transform = rustforge_core::math::Transform {
            position: Vec3::new(1.0, 2.0, 3.0),
            rotation: Quat::from_rotation_y(1.57),
            scale: Vec3::splat(2.0),
        };

        let data = TransformData::from(&transform);
        let back: rustforge_core::math::Transform = data.into();

        assert_eq!(back.position, transform.position);
        assert!((back.rotation.dot(transform.rotation) - 1.0).abs() < 0.001);
        assert_eq!(back.scale, transform.scale);
    }

    #[test]
    fn test_scene_file_serialization() {
        let scene_file = SceneFile {
            version: "1.0".to_string(),
            nodes: vec![
                NodeData {
                    name: "Player".to_string(),
                    transform: TransformData::default(),
                    children: vec!["Weapon".to_string()],
                    components: vec![ComponentData::Mesh {
                        path: "models/player.obj".to_string(),
                        cast_shadows: true,
                    }],
                    tags: vec!["player".to_string()],
                    active: true,
                },
                NodeData {
                    name: "Weapon".to_string(),
                    transform: TransformData {
                        position: [0.5, 0.0, 0.0],
                        ..Default::default()
                    },
                    children: vec![],
                    components: vec![],
                    tags: vec![],
                    active: true,
                },
            ],
            assets: AssetReferences::default(),
        };

        // Test JSON serialization
        let json = serde_json::to_string_pretty(&scene_file).unwrap();
        let from_json: SceneFile = serde_json::from_str(&json).unwrap();
        assert_eq!(from_json.nodes.len(), 2);
        assert_eq!(from_json.nodes[0].name, "Player");

        // Test RON serialization
        let ron = ron::ser::to_string_pretty(&scene_file, Default::default()).unwrap();
        let from_ron: SceneFile = ron::from_str(&ron).unwrap();
        assert_eq!(from_ron.nodes.len(), 2);
    }
}
