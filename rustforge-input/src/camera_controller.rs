//! Camera controller for free-moving cameras

use glam::{Quat, Vec3};
use rustforge_graphics::Camera;
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

/// Controls for a free-moving camera
#[derive(Debug, Clone)]
pub struct CameraController {
    // Movement settings
    pub move_speed: f32,
    pub fast_speed: f32,
    pub mouse_sensitivity: f32,

    // Camera state
    pub yaw: f32,
    pub pitch: f32,

    // Input state
    movement: MovementState,
    mouse_pressed: bool,
    last_mouse_pos: (f32, f32),
    shift_pressed: bool,
}

#[derive(Debug, Clone, Default)]
struct MovementState {
    forward: bool,
    backward: bool,
    left: bool,
    right: bool,
    up: bool,
    down: bool,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            move_speed: 5.0,
            fast_speed: 15.0,
            mouse_sensitivity: 0.005,
            yaw: 0.0,
            pitch: 0.0,
            movement: MovementState::default(),
            mouse_pressed: false,
            last_mouse_pos: (0.0, 0.0),
            shift_pressed: false,
        }
    }
}

impl CameraController {
    pub fn new(move_speed: f32, mouse_sensitivity: f32) -> Self {
        Self {
            move_speed,
            fast_speed: move_speed * 3.0,
            mouse_sensitivity,
            ..Default::default()
        }
    }

    /// Handle window events. Returns true if event was handled.
    pub fn handle_event(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput { event, .. } => {
                self.handle_keyboard(event.physical_key, event.state)
            }
            WindowEvent::MouseInput { state, button, .. } => {
                self.handle_mouse_button(*button, *state)
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.handle_mouse_motion((position.x as f32, position.y as f32));
                true
            }
            _ => false,
        }
    }

    /// Update the camera based on input state
    pub fn update(&mut self, camera: &mut Camera, dt: f32) {
        // Update rotation
        camera.transform.rotation =
            Quat::from_rotation_y(self.yaw) * Quat::from_rotation_x(self.pitch);

        // Calculate movement
        let forward = camera.transform.rotation * Vec3::NEG_Z;
        let right = camera.transform.rotation * Vec3::X;

        let mut velocity = Vec3::ZERO;

        if self.movement.forward {
            velocity += forward;
        }
        if self.movement.backward {
            velocity -= forward;
        }
        if self.movement.right {
            velocity += right;
        }
        if self.movement.left {
            velocity -= right;
        }
        if self.movement.up {
            velocity += Vec3::Y;
        }
        if self.movement.down {
            velocity -= Vec3::Y;
        }

        if velocity.length_squared() > 0.0 {
            let speed = if self.shift_pressed {
                self.fast_speed
            } else {
                self.move_speed
            };
            velocity = velocity.normalize() * speed * dt;
            camera.transform.position += velocity;
        }
    }

    fn handle_keyboard(&mut self, key: PhysicalKey, state: ElementState) -> bool {
        let is_pressed = state == ElementState::Pressed;

        match key {
            PhysicalKey::Code(KeyCode::KeyW) | PhysicalKey::Code(KeyCode::ArrowUp) => {
                self.movement.forward = is_pressed;
                true
            }
            PhysicalKey::Code(KeyCode::KeyS) | PhysicalKey::Code(KeyCode::ArrowDown) => {
                self.movement.backward = is_pressed;
                true
            }
            PhysicalKey::Code(KeyCode::KeyA) | PhysicalKey::Code(KeyCode::ArrowLeft) => {
                self.movement.left = is_pressed;
                true
            }
            PhysicalKey::Code(KeyCode::KeyD) | PhysicalKey::Code(KeyCode::ArrowRight) => {
                self.movement.right = is_pressed;
                true
            }
            PhysicalKey::Code(KeyCode::Space) => {
                self.movement.up = is_pressed;
                true
            }
            PhysicalKey::Code(KeyCode::KeyC) | PhysicalKey::Code(KeyCode::ControlLeft) => {
                self.movement.down = is_pressed;
                true
            }
            PhysicalKey::Code(KeyCode::ShiftLeft) | PhysicalKey::Code(KeyCode::ShiftRight) => {
                self.shift_pressed = is_pressed;
                true
            }
            _ => false,
        }
    }

    fn handle_mouse_button(&mut self, button: MouseButton, state: ElementState) -> bool {
        if button == MouseButton::Left {
            self.mouse_pressed = state == ElementState::Pressed;
            true
        } else {
            false
        }
    }

    fn handle_mouse_motion(&mut self, position: (f32, f32)) {
        if self.mouse_pressed {
            let delta_x = position.0 - self.last_mouse_pos.0;
            let delta_y = position.1 - self.last_mouse_pos.1;

            self.yaw -= delta_x * self.mouse_sensitivity;
            self.pitch -= delta_y * self.mouse_sensitivity;

            // Clamp pitch to prevent camera flipping
            self.pitch = self
                .pitch
                .clamp(-89.0f32.to_radians(), 89.0f32.to_radians());
        }
        self.last_mouse_pos = position;
    }

    /// Reset camera controller state
    pub fn reset(&mut self) {
        self.yaw = 0.0;
        self.pitch = 0.0;
        self.movement = MovementState::default();
        self.mouse_pressed = false;
        self.shift_pressed = false;
    }
}
