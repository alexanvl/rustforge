//! Graphics example demonstrating material creation and properties

use glam::Vec3;
use rustforge_graphics::prelude::*;

fn main() {
    println!("RustForge Graphics - Material Demo");
    println!("==================================");

    // Create a default material
    let default_material = Material::default();
    println!("Default material: {:?}", default_material);
    println!("Albedo: {:?}", default_material.albedo);
    println!(
        "Metallic: {:.2}, Roughness: {:.2}",
        default_material.metallic, default_material.roughness
    );

    // Create a colored material
    let red_material = Material::colored("Red Material", Vec3::new(1.0, 0.0, 0.0));
    println!("\nRed material: {:?}", red_material);
    println!("Albedo: {:?}", red_material.albedo);

    // Create a metallic material
    let gold_material = Material::metallic(
        "Gold Material",
        Vec3::new(1.0, 0.8, 0.2), // Gold color
        0.9,                      // High metallic
        0.1,                      // Low roughness (shiny)
    );
    println!("\nGold material: {:?}", gold_material);
    println!("Albedo: {:?}", gold_material.albedo);
    println!(
        "Metallic: {:.2}, Roughness: {:.2}",
        gold_material.metallic, gold_material.roughness
    );

    // Create a rough material
    let concrete_material = Material::metallic(
        "Concrete Material",
        Vec3::new(0.5, 0.5, 0.5), // Gray color
        0.0,                      // Non-metallic
        0.8,                      // High roughness (rough)
    );
    println!("\nConcrete material: {:?}", concrete_material);
    println!(
        "Metallic: {:.2}, Roughness: {:.2}",
        concrete_material.metallic, concrete_material.roughness
    );

    // Demonstrate material property ranges
    println!("\nMaterial Property Ranges");
    println!("========================");

    let materials = vec![
        ("Default", Material::default()),
        ("Red", Material::colored("Red", Vec3::new(1.0, 0.0, 0.0))),
        (
            "Gold",
            Material::metallic("Gold", Vec3::new(1.0, 0.8, 0.2), 0.9, 0.1),
        ),
        (
            "Concrete",
            Material::metallic("Concrete", Vec3::new(0.5, 0.5, 0.5), 0.0, 0.8),
        ),
    ];

    for (name, material) in materials {
        println!(
            "{}: Metallic={:.2}, Roughness={:.2}, Emissive={:?}",
            name, material.metallic, material.roughness, material.emissive
        );
    }

    // Test texture assignment
    println!("\nTexture Assignment");
    println!("==================");

    let mut textured_material = Material::new("Textured Material");
    textured_material.albedo_texture = Some("albedo.png".to_string());
    textured_material.normal_texture = Some("normal.png".to_string());
    textured_material.metallic_roughness_texture = Some("metallic_roughness.png".to_string());

    println!("Textured material: {:?}", textured_material);
    println!("Albedo texture: {:?}", textured_material.albedo_texture);
    println!("Normal texture: {:?}", textured_material.normal_texture);
    println!(
        "Metallic-roughness texture: {:?}",
        textured_material.metallic_roughness_texture
    );

    // Demonstrate emissive materials
    println!("\nEmissive Materials");
    println!("==================");

    let mut emissive_material = Material::colored("Emissive Light", Vec3::new(1.0, 1.0, 0.8));
    emissive_material.emissive = Vec3::new(2.0, 2.0, 1.0); // Bright white light
    println!("Emissive material: {:?}", emissive_material);
    println!("Emissive color: {:?}", emissive_material.emissive);
}
