//! RustForge Game Engine

use std::sync::Arc;
use winit::{
    event::{Event, WindowEvent, ElementState},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
    keyboard::KeyCode,
};
use specs::WorldExt;

use rustforge_core::prelude::*;
use rustforge_ecs::prelude::*;
use rustforge_graphics::prelude::*;
use rustforge_physics::prelude::*;
use rustforge_input::InputState;
use rustforge_audio::AudioManager;

pub mod systems;

/// Main engine struct
pub struct Engine {
    _window: Arc<Window>,
    pub ecs_world: EcsWorld,
    _physics_world: Arc<std::sync::Mutex<PhysicsWorld>>,
    // TODO: Re-enable once raw-window-handle compatibility is fixed
    // renderer: Renderer,
    input_state: InputState,
    _audio_manager: AudioManager,
    running: bool,
}

impl Engine {
    /// Create new engine instance
    pub fn new(event_loop: &EventLoop<()>) -> Result<Self> {
        // Initialize logging
        env_logger::init();

        // Create window
        let window = Arc::new(
            event_loop.create_window(
                winit::window::WindowAttributes::default()
                    .with_title("RustForge Engine")
                    .with_inner_size(winit::dpi::LogicalSize::new(1280, 720))
            )
            .map_err(|e| Error::Init(format!("Failed to create window: {}", e)))?
        );

        // Initialize subsystems
        let mut ecs_world = EcsWorld::new()?;
        let physics_world = Arc::new(std::sync::Mutex::new(PhysicsWorld::new(Vec3::new(0.0, -9.81, 0.0))));
        // TODO: Re-enable renderer once raw-window-handle compatibility is fixed
        // let renderer = Renderer::new(window.clone())?;
        let input_state = InputState::new();
        let audio_manager = AudioManager::new()?;

        // Register physics components
        ecs_world.world_mut().register::<PhysicsBody>();
        ecs_world.world_mut().register::<Collider>();
        ecs_world.world_mut().register::<Force>();
        ecs_world.world_mut().register::<Impulse>();
        ecs_world.world_mut().register::<PhysicsSync>();

        // Register graphics components
        ecs_world.world_mut().register::<Renderable>();
        ecs_world.world_mut().register::<Light>();
        ecs_world.world_mut().register::<CameraComponent>();

        // Register gameplay components
        ecs_world.world_mut().register::<PlayerControlled>();

        // Insert global resources
        ecs_world.world_mut().insert(physics_world.clone());
        // TODO: Re-enable once renderer is fixed
        // ecs_world.world_mut().insert(Arc::new(renderer));
        ecs_world.world_mut().insert(input_state.clone());

        Ok(Self {
            _window: window,
            ecs_world,
            _physics_world: physics_world,
            // TODO: Re-enable once renderer is fixed
            // renderer,
            input_state,
            _audio_manager: audio_manager,
            running: true,
        })
    }

    /// Run the engine
    pub fn run(mut self, event_loop: EventLoop<()>) {
        event_loop.run(move |event, event_loop_window_target| {
            event_loop_window_target.set_control_flow(ControlFlow::Poll);

            match event {
                Event::WindowEvent { event, .. } => {
                    self.handle_window_event(event);
                }
                Event::NewEvents(_) => {
                    if self.running {
                        self.update();
                        self.render();
                    }
                }
                _ => {}
            }
        });
    }

    /// Handle window events
    fn handle_window_event(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                self.running = false;
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let winit::keyboard::PhysicalKey::Code(key) = event.physical_key {
                    self.input_state.handle_keyboard(key, event.state);

                    // ESC to quit
                    if key == KeyCode::Escape && event.state == ElementState::Pressed {
                        self.running = false;
                    }
                }
            }
            WindowEvent::MouseInput { button, state, .. } => {
                self.input_state.handle_mouse_button(button, state);
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.input_state.handle_mouse_motion(position);
            }
            WindowEvent::MouseWheel { delta, .. } => {
                use winit::event::MouseScrollDelta;
                match delta {
                    MouseScrollDelta::LineDelta(_, y) => {
                        self.input_state.handle_mouse_wheel(y);
                    }
                    MouseScrollDelta::PixelDelta(pos) => {
                        self.input_state.handle_mouse_wheel(pos.y as f32 * 0.1);
                    }
                }
            }
            _ => {}
        }
    }

    /// Update engine state
    fn update(&mut self) {
        // Update ECS (which updates time)
        self.ecs_world.update();

        // Run generic engine systems
        use specs::RunNow;
        let mut velocity_system = systems::VelocitySystem;
        velocity_system.run_now(&self.ecs_world.world());

        // Fixed timestep physics updates
        let mut time = self.ecs_world.world_mut().write_resource::<Time>();
        let mut fixed_updates = 0;
        while time.should_fixed_update() && fixed_updates < 3 {
            drop(time); // Release time resource

            // Run physics systems
            self.fixed_update();
            fixed_updates += 1;

            time = self.ecs_world.world_mut().write_resource::<Time>();
        }
        drop(time);

        // Update input state resource
        *self.ecs_world.world_mut().write_resource::<InputState>() = self.input_state.clone();

        // Clear per-frame input state
        self.input_state.clear_frame_state();
    }

    /// Fixed timestep update for physics
    fn fixed_update(&mut self) {
        use specs::RunNow;
        use rustforge_physics::systems::{
            PhysicsInitSystem, PhysicsForceSystem, PhysicsStepSystem, PhysicsSyncSystem
        };

        // Run physics systems in order
        let mut init_system = PhysicsInitSystem;
        init_system.run_now(&self.ecs_world.world());

        let mut force_system = PhysicsForceSystem;
        force_system.run_now(&self.ecs_world.world());

        let mut step_system = PhysicsStepSystem;
        step_system.run_now(&self.ecs_world.world());

        let mut sync_system = PhysicsSyncSystem;
        sync_system.run_now(&self.ecs_world.world());

        // Maintain world after systems
        self.ecs_world.world_mut().maintain();
    }

    /// Render frame
    fn render(&mut self) {
        // TODO: Re-enable rendering once Vulkan compatibility is fixed
        // let _ = self.renderer.begin_frame();
        // let _ = self.renderer.end_frame();

        // For now, just print a debug message occasionally
        use std::sync::atomic::{AtomicU64, Ordering};
        static FRAME_COUNT: AtomicU64 = AtomicU64::new(0);

        let count = FRAME_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
        if count % 300 == 0 {
            println!("Engine running... Frame {}", count);
        }
    }
}

/// Re-export commonly used types
pub mod prelude {
    pub use super::Engine;
    pub use rustforge_core::prelude::*;
    pub use rustforge_ecs::prelude::*;
    pub use rustforge_graphics::prelude::*;
    pub use rustforge_physics::prelude::*;
    pub use rustforge_input::InputState;
}
