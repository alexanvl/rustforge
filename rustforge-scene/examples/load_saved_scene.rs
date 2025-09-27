//! Example that loads the scene saved by basic_scene example

use rustforge_scene::prelude::*;

fn main() -> Result<()> {
    println!("Loading saved scenes...\n");

    // Try to load the JSON version
    let json_path = "examples/assets/basic_scene.json";
    match SceneGraph::load_json(json_path) {
        Ok((scene, scene_file)) => {
            println!("Successfully loaded JSON scene from: {}", json_path);
            println!("Scene file version: {}", scene_file.version);
            println!("Number of nodes: {}", scene_file.nodes.len());

            println!("\nScene Hierarchy (from JSON):");
            print_hierarchy(&scene, scene.root(), 0);

            // Update transforms to get correct world positions
            let mut scene = scene;
            scene.update_transforms();

            // Test finding nodes
            if let Some(player_id) = scene.find_by_name("Player") {
                let player = scene.get(player_id)?;
                println!("\nPlayer found at position: {:?}", player.world_position());
            }
        }
        Err(e) => {
            eprintln!("Failed to load JSON scene: {}", e);
            eprintln!(
                "Make sure to run the 'basic_scene' example first to generate the scene file."
            );
        }
    }

    println!("\n{}\n", "=".repeat(50));

    // Try to load the RON version
    let ron_path = "examples/assets/basic_scene.ron";
    match SceneGraph::load_ron(ron_path) {
        Ok((scene, scene_file)) => {
            println!("Successfully loaded RON scene from: {}", ron_path);
            println!("Scene file version: {}", scene_file.version);
            println!("Number of nodes: {}", scene_file.nodes.len());

            println!("\nScene Hierarchy (from RON):");
            print_hierarchy(&scene, scene.root(), 0);

            // Update transforms to get correct world positions
            let mut scene = scene;
            scene.update_transforms();

            // Demonstrate querying nodes by tags
            println!("\nNodes with 'light' tag:");
            for (_id, node) in scene.iter() {
                if node.tags.contains("light") {
                    println!("  - {} at position {:?}", node.name, node.world_position());
                }
            }

            println!("\nNodes with 'dynamic' tag:");
            for (_id, node) in scene.iter() {
                if node.tags.contains("dynamic") {
                    println!("  - {} at position {:?}", node.name, node.world_position());
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to load RON scene: {}", e);
            eprintln!(
                "Make sure to run the 'basic_scene' example first to generate the scene file."
            );
        }
    }

    Ok(())
}

fn print_hierarchy(scene: &SceneGraph, node_id: NodeId, depth: usize) {
    let node = scene.get(node_id).unwrap();

    if depth > 0 {
        // Skip root node
        let indent = "  ".repeat(depth - 1);
        println!(
            "{}- {} (tags: {:?})",
            indent,
            node.name,
            node.tags.iter().collect::<Vec<_>>()
        );
    }

    for &child_id in node.children() {
        print_hierarchy(scene, child_id, depth + 1);
    }
}
