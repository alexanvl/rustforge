//! Basic scene graph usage example

use glam::{Quat, Vec3};
use rustforge_scene::prelude::*;

fn main() {
    // Create a scene graph
    let mut scene = SceneGraph::new();

    // Create a simple scene hierarchy:
    // - Environment
    //   - Sun (light)
    //   - Terrain
    // - Player
    //   - Camera
    //   - Weapon

    // Add environment group
    let environment = scene.add_node(SceneNode::new("Environment").with_tags(["environment"]));
    scene.add_child(scene.root(), environment).unwrap();

    // Add sun
    let sun = scene.add_node(SceneNode::new("Sun").with_tags(["light", "directional"]));
    scene.add_child(environment, sun).unwrap();
    scene.set_position(sun, Vec3::new(0.0, 100.0, 0.0)).unwrap();
    scene
        .set_rotation(sun, Quat::from_rotation_x(-0.7854))
        .unwrap(); // 45 degrees down

    // Add terrain
    let terrain = scene.add_node(SceneNode::new("Terrain").with_tags(["static", "collidable"]));
    scene.add_child(environment, terrain).unwrap();

    // Add player
    let player = scene.add_node(SceneNode::new("Player").with_tags(["player", "dynamic"]));
    scene.add_child(scene.root(), player).unwrap();
    scene
        .set_position(player, Vec3::new(0.0, 1.0, 0.0))
        .unwrap();

    // Add camera to player
    let camera = scene.add_node(SceneNode::new("Camera").with_tags(["camera"]));
    scene.add_child(player, camera).unwrap();
    scene
        .set_position(camera, Vec3::new(0.0, 0.6, 0.0))
        .unwrap();

    // Add weapon to player
    let weapon = scene.add_node(SceneNode::new("Weapon").with_tags(["weapon", "item"]));
    scene.add_child(player, weapon).unwrap();
    scene
        .set_position(weapon, Vec3::new(0.3, 0.0, 0.5))
        .unwrap();

    // Update all transforms
    scene.update_transforms();

    // Print the scene hierarchy
    println!("Scene Hierarchy:");
    print_hierarchy(&scene, scene.root(), 0);

    // Demonstrate finding nodes
    if let Some(player_id) = scene.find_by_name("Player") {
        let player_node = scene.get(player_id).unwrap();
        println!(
            "\nPlayer world position: {:?}",
            player_node.world_position()
        );
    }

    if let Some(weapon_id) = scene.find_by_name("Weapon") {
        let weapon_node = scene.get(weapon_id).unwrap();
        println!("Weapon world position: {:?}", weapon_node.world_position());
    }

    // Demonstrate spatial query with a frustum
    let view = glam::Mat4::look_at_rh(Vec3::new(5.0, 5.0, 5.0), Vec3::ZERO, Vec3::Y);
    let proj = glam::Mat4::perspective_rh(1.0472, 1.0, 0.1, 100.0); // 60 degrees FOV
    let frustum = Frustum::from_matrix(&(proj * view));

    println!("\nNodes in frustum:");
    let visible_nodes = scene.query_frustum(&frustum);
    for node_id in visible_nodes {
        let node = scene.get(node_id).unwrap();
        println!("  - {}", node.name);
    }

    // Save the scene to JSON in examples/assets directory
    let scene_path = std::path::Path::new("examples/assets/basic_scene.json");
    if let Err(e) = scene.save_json(&scene_path) {
        eprintln!("Failed to save scene: {}", e);
    } else {
        println!("\nScene saved to: {}", scene_path.display());

        // Also save a RON version
        let ron_path = std::path::Path::new("examples/assets/basic_scene.ron");
        if let Err(e) = scene.save_ron(&ron_path) {
            eprintln!("Failed to save RON scene: {}", e);
        } else {
            println!("RON version saved to: {}", ron_path.display());
        }
    }
}

fn print_hierarchy(scene: &SceneGraph, node_id: NodeId, depth: usize) {
    let node = scene.get(node_id).unwrap();
    let indent = "  ".repeat(depth);

    println!(
        "{}- {} (tags: {:?})",
        indent,
        node.name,
        node.tags.iter().collect::<Vec<_>>()
    );

    for &child_id in node.children() {
        print_hierarchy(scene, child_id, depth + 1);
    }
}
