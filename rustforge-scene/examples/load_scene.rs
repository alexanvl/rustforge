//! Example of loading a scene from a file

use rustforge_scene::prelude::*;

fn main() -> Result<()> {
    // Example scene data - normally this would be loaded from a file
    let scene_json = r#"{
        "version": "1.0",
        "nodes": [
            {
                "name": "Level1",
                "transform": {
                    "position": [0.0, 0.0, 0.0],
                    "rotation": [0.0, 0.0, 0.0, 1.0],
                    "scale": [1.0, 1.0, 1.0]
                },
                "children": ["StartArea", "EnemySpawn1"],
                "components": [],
                "tags": ["level", "root"],
                "active": true
            },
            {
                "name": "StartArea",
                "transform": {
                    "position": [0.0, 0.0, 0.0]
                },
                "children": ["PlayerSpawn", "Checkpoint1"],
                "components": [
                    {
                        "type": "Mesh",
                        "path": "models/start_area.obj",
                        "cast_shadows": true
                    }
                ],
                "tags": ["area", "start"],
                "active": true
            },
            {
                "name": "PlayerSpawn",
                "transform": {
                    "position": [0.0, 1.0, 0.0]
                },
                "children": [],
                "components": [],
                "tags": ["spawn", "player"],
                "active": true
            },
            {
                "name": "Checkpoint1",
                "transform": {
                    "position": [10.0, 0.0, 0.0]
                },
                "children": [],
                "components": [
                    {
                        "type": "Custom",
                        "component_type": "Checkpoint",
                        "id": 1,
                        "respawn_offset": [0.0, 2.0, 0.0]
                    }
                ],
                "tags": ["checkpoint"],
                "active": true
            },
            {
                "name": "EnemySpawn1",
                "transform": {
                    "position": [20.0, 0.0, 5.0]
                },
                "children": [],
                "components": [
                    {
                        "type": "Custom",
                        "component_type": "EnemySpawner",
                        "enemy_type": "goblin",
                        "spawn_rate": 5.0,
                        "max_enemies": 3
                    }
                ],
                "tags": ["spawn", "enemy"],
                "active": true
            }
        ],
        "assets": {
            "meshes": {
                "start_area": "models/start_area.obj"
            }
        }
    }"#;

    // Parse the scene file
    let scene_file: SceneFile = serde_json::from_str(scene_json)?;
    println!("Loaded scene file version: {}", scene_file.version);
    println!("Number of nodes: {}", scene_file.nodes.len());

    // Create scene graph from the file
    let scene = SceneGraph::from_scene_file(&scene_file)?;

    // Print hierarchy
    println!("\nScene Hierarchy:");
    print_hierarchy(&scene, scene.root(), 0);

    // Find specific nodes
    if let Some(player_spawn) = scene.find_by_name("PlayerSpawn") {
        let node = scene.get(player_spawn)?;
        println!("\nPlayer spawn position: {:?}", node.world_position());
    }

    // Query nodes by tags
    println!("\nSpawn points:");
    for (id, node) in scene.iter() {
        if node.tags.contains("spawn") {
            println!(
                "  - {} at position {:?}",
                node.name, node.transform.position
            );
        }
    }

    // Look at component data
    println!("\nComponent data:");
    for node_data in &scene_file.nodes {
        if !node_data.components.is_empty() {
            println!("  {}:", node_data.name);
            for component in &node_data.components {
                match component {
                    ComponentData::Mesh { path, cast_shadows } => {
                        println!("    - Mesh: {} (shadows: {})", path, cast_shadows);
                    }
                    ComponentData::Custom {
                        component_type,
                        data,
                    } => {
                        println!("    - Custom {}: {:?}", component_type, data);
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

fn print_hierarchy(scene: &SceneGraph, node_id: NodeId, depth: usize) {
    if depth == 0 {
        // Skip printing root node
        for (id, node) in scene.iter() {
            if node.parent() == Some(scene.root()) {
                print_node_recursive(scene, id, node, 0);
            }
        }
    }
}

fn print_node_recursive(scene: &SceneGraph, node_id: NodeId, node: &SceneNode, depth: usize) {
    let indent = "  ".repeat(depth);
    println!("{}- {}", indent, node.name);

    for &child_id in node.children() {
        if let Ok(child) = scene.get(child_id) {
            print_node_recursive(scene, child_id, child, depth + 1);
        }
    }
}
