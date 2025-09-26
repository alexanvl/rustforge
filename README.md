# RustForge Game Engine

A custom 3D game engine built from scratch in Rust, emphasizing modularity, performance, and safety through Rust's ownership and concurrency models.

## Features

- **Entity-Component-System (ECS)** architecture for data-oriented design
- **Vulkan-based rendering** for modern 3D graphics
- **Physics simulation** using Rapier3D
- **Cross-platform support** for Windows, Linux, and macOS
- **Modular architecture** with separate crates for each subsystem

## Architecture

The engine is organized as a Cargo workspace with the following modules:

- `rustforge-core`: Core types, math utilities, and error handling
- `rustforge-ecs`: Entity-Component-System implementation using specs
- `rustforge-graphics`: Vulkan-based rendering system
- `rustforge-physics`: Physics simulation using Rapier3D
- `rustforge-input`: Input handling for keyboard and mouse
- `rustforge-audio`: Audio system (placeholder)
- `rustforge-engine`: Main engine integration

## Requirements

- Rust 2021 edition or later
- Vulkan SDK installed
- Cargo and standard Rust toolchain

## Building

```bash
# Clone the repository
git clone https://github.com/yourusername/rustforge.git
cd rustforge

# Build all crates
cargo build --release

# Run tests for all modules
cargo test

# Run the 3D graphics demo
cargo run -p sphere-demo

# Run the physics simulation demo
cargo run -p physics-console-demo
```

## Usage

Create a new game by depending on the engine crate:

```rust
use rustforge_engine::prelude::*;
use winit::event_loop::EventLoop;

fn main() -> Result<()> {
    let event_loop = EventLoop::new();
    let engine = Engine::new(&event_loop)?;

    // Set up your game scene here

    engine.run(event_loop);
    Ok(())
}
```

## Demos

### Working Examples

**Graphics Demo (`sphere-demo`)**
- 3D rotating sphere with proper geometry
- Phong lighting model (ambient, diffuse, specular)
- Point light source with configurable properties
- Camera orbiting around the sphere
- Material system with PBR properties
- Real-time rendering at 60 FPS using wgpu

**Physics Demo (`physics-console-demo`)**
- Real-time physics simulation with ASCII visualization
- Gravity, collision detection, and bouncing objects
- Multiple physics entities with different properties
- Ground collision and wall bouncing
- Demonstrates physics system working correctly

### Module Examples

Each individual crate includes focused examples:

**Core Module (`rustforge-core`)**
- `basic_math.rs` - Transform operations and time management

**Graphics Module (`rustforge-graphics`)**
- `camera_demo.rs` - Camera matrices and transformations
- `mesh_demo.rs` - Mesh creation and vertex data
- `material_demo.rs` - Material properties and PBR values

**Physics Module (`rustforge-physics`)**
- `collider_demo.rs` - Different collider shapes and properties
- `rigid_body_demo.rs` - Rigid body types and behaviors
- `physics_world_demo.rs` - World simulation and entity management

## Current Status

This is v1 of the engine focusing on:
- Modular architecture with clean separation of concerns
- Working 3D graphics pipeline with modern rendering
- Physics simulation with real-time collision detection
- Comprehensive unit testing across all modules
- Minimal, focused examples for each component

## Future Enhancements

- Complete Vulkan rendering pipeline
- Asset loading system
- Networking support
- Advanced rendering features (shadows, post-processing)
- Web platform support via WebGPU

## License

Licensed under either MIT or Apache-2.0 at your option.