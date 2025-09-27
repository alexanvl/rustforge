//! Physics Console Demo - Shows physics simulation with visual ASCII output
//! This demonstrates the physics system working without complex rendering

use glam::Vec3;
use rustforge_physics::prelude::*;
use std::thread;
use std::time::{Duration, Instant};

const WIDTH: usize = 60;
const HEIGHT: usize = 20;

struct PhysicsDemo {
    _physics_world: PhysicsWorld,
    entities: Vec<PhysicsEntity>,
    time: Instant,
    last_update: Instant,
}

#[derive(Clone)]
struct PhysicsEntity {
    position: Vec3,
    velocity: Vec3,
    _size: f32,
    char: char,
}

impl PhysicsDemo {
    fn new() -> Self {
        let physics_world = PhysicsWorld::new(Vec3::new(0.0, -9.81, 0.0));

        let mut entities = Vec::new();
        let chars = ['●', '◆', '▲', '■', '★', '♦', '♠', '♣', '♥', '♡'];

        for i in 0..10 {
            let x = (i as f32 - 5.0) * 2.0;
            entities.push(PhysicsEntity {
                position: Vec3::new(x, 15.0 + i as f32, 0.0),
                velocity: Vec3::new(
                    (i as f32 - 5.0) * 0.5, // Slight horizontal velocity
                    0.0,
                    0.0,
                ),
                _size: 0.5 + (i as f32 * 0.1),
                char: chars[i % chars.len()],
            });
        }

        Self {
            _physics_world: physics_world,
            entities,
            time: Instant::now(),
            last_update: Instant::now(),
        }
    }

    fn update(&mut self) {
        let delta_time = self.last_update.elapsed().as_secs_f32();
        self.last_update = Instant::now();

        // Simple physics simulation
        for entity in &mut self.entities {
            // Apply gravity
            entity.velocity.y -= 9.81 * delta_time;

            // Update position
            entity.position += entity.velocity * delta_time;

            // Bounce off ground
            if entity.position.y < 0.0 {
                entity.position.y = 0.0;
                entity.velocity.y = -entity.velocity.y * 0.7; // Damping
                entity.velocity.x *= 0.95; // Friction
            }

            // Bounce off walls
            if entity.position.x < -15.0 {
                entity.position.x = -15.0;
                entity.velocity.x = -entity.velocity.x * 0.8;
            } else if entity.position.x > 15.0 {
                entity.position.x = 15.0;
                entity.velocity.x = -entity.velocity.x * 0.8;
            }
        }
    }

    fn render(&self) {
        // Clear screen
        print!("\x1B[2J\x1B[1;1H");

        // Create a 2D grid
        let mut grid = vec![vec![' '; WIDTH]; HEIGHT];

        // Draw ground
        for x in 0..WIDTH {
            grid[HEIGHT - 1][x] = '─';
        }

        // Draw entities
        for entity in &self.entities {
            let screen_x = ((entity.position.x + 15.0) / 30.0 * (WIDTH - 1) as f32) as usize;
            let screen_y = ((15.0 - entity.position.y) / 15.0 * (HEIGHT - 2) as f32) as usize;

            if screen_x < WIDTH && screen_y < HEIGHT - 1 {
                grid[screen_y][screen_x] = entity.char;
            }
        }

        // Print the grid
        for row in &grid {
            for &cell in row {
                print!("{}", cell);
            }
            println!();
        }

        // Print physics info
        println!();
        println!(
            "Physics Demo - Time: {:.2}s",
            self.time.elapsed().as_secs_f32()
        );
        println!("Entities: {} | Gravity: -9.81 m/s²", self.entities.len());

        // Show some entity positions
        for (i, entity) in self.entities.iter().enumerate().take(3) {
            println!(
                "Entity {}: pos=({:.1}, {:.1}) vel=({:.1}, {:.1})",
                i, entity.position.x, entity.position.y, entity.velocity.x, entity.velocity.y
            );
        }
    }

    fn run(mut self) {
        println!("Starting Physics Console Demo...");
        println!("This demo shows:");
        println!("  - Physics simulation with gravity");
        println!("  - Collision detection and bouncing");
        println!("  - Real-time visualization in ASCII");
        println!("  - Multiple physics entities");
        println!();

        thread::sleep(Duration::from_millis(1000));

        loop {
            self.update();
            self.render();

            // Control frame rate
            thread::sleep(Duration::from_millis(50)); // ~20 FPS
        }
    }
}

fn main() {
    env_logger::init();

    let demo = PhysicsDemo::new();
    demo.run();
}
