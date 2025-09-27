//! Input handling module for RustForge

use std::collections::HashSet;
use winit::event::{ElementState, MouseButton};
use winit::keyboard::KeyCode;
use winit::dpi::PhysicalPosition;
use glam::Vec2;

/// Input state tracking
#[derive(Debug, Clone)]
pub struct InputState {
    // Keyboard
    keys_pressed: HashSet<KeyCode>,
    keys_just_pressed: HashSet<KeyCode>,
    keys_just_released: HashSet<KeyCode>,

    // Mouse
    mouse_buttons_pressed: HashSet<MouseButton>,
    mouse_buttons_just_pressed: HashSet<MouseButton>,
    mouse_buttons_just_released: HashSet<MouseButton>,
    mouse_position: Vec2,
    mouse_delta: Vec2,
    mouse_wheel_delta: f32,
}

impl Default for InputState {
    fn default() -> Self {
        Self::new()
    }
}

impl InputState {
    pub fn new() -> Self {
        Self {
            keys_pressed: HashSet::new(),
            keys_just_pressed: HashSet::new(),
            keys_just_released: HashSet::new(),
            mouse_buttons_pressed: HashSet::new(),
            mouse_buttons_just_pressed: HashSet::new(),
            mouse_buttons_just_released: HashSet::new(),
            mouse_position: Vec2::ZERO,
            mouse_delta: Vec2::ZERO,
            mouse_wheel_delta: 0.0,
        }
    }

    /// Clear per-frame input state
    pub fn clear_frame_state(&mut self) {
        self.keys_just_pressed.clear();
        self.keys_just_released.clear();
        self.mouse_buttons_just_pressed.clear();
        self.mouse_buttons_just_released.clear();
        self.mouse_delta = Vec2::ZERO;
        self.mouse_wheel_delta = 0.0;
    }

    /// Handle keyboard input
    pub fn handle_keyboard(&mut self, key: KeyCode, state: ElementState) {
        match state {
            ElementState::Pressed => {
                if self.keys_pressed.insert(key) {
                    self.keys_just_pressed.insert(key);
                }
            }
            ElementState::Released => {
                if self.keys_pressed.remove(&key) {
                    self.keys_just_released.insert(key);
                }
            }
        }
    }

    /// Handle mouse button input
    pub fn handle_mouse_button(&mut self, button: MouseButton, state: ElementState) {
        match state {
            ElementState::Pressed => {
                if self.mouse_buttons_pressed.insert(button) {
                    self.mouse_buttons_just_pressed.insert(button);
                }
            }
            ElementState::Released => {
                if self.mouse_buttons_pressed.remove(&button) {
                    self.mouse_buttons_just_released.insert(button);
                }
            }
        }
    }

    /// Handle mouse movement
    pub fn handle_mouse_motion(&mut self, position: PhysicalPosition<f64>) {
        let new_pos = Vec2::new(position.x as f32, position.y as f32);
        self.mouse_delta = new_pos - self.mouse_position;
        self.mouse_position = new_pos;
    }

    /// Handle mouse wheel
    pub fn handle_mouse_wheel(&mut self, delta: f32) {
        self.mouse_wheel_delta += delta;
    }

    // Query methods
    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.keys_pressed.contains(&key)
    }

    pub fn is_key_just_pressed(&self, key: KeyCode) -> bool {
        self.keys_just_pressed.contains(&key)
    }

    pub fn is_key_just_released(&self, key: KeyCode) -> bool {
        self.keys_just_released.contains(&key)
    }

    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.mouse_buttons_pressed.contains(&button)
    }

    pub fn is_mouse_button_just_pressed(&self, button: MouseButton) -> bool {
        self.mouse_buttons_just_pressed.contains(&button)
    }

    pub fn is_mouse_button_just_released(&self, button: MouseButton) -> bool {
        self.mouse_buttons_just_released.contains(&button)
    }

    pub fn mouse_position(&self) -> Vec2 {
        self.mouse_position
    }

    pub fn mouse_delta(&self) -> Vec2 {
        self.mouse_delta
    }

    pub fn mouse_wheel_delta(&self) -> f32 {
        self.mouse_wheel_delta
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use winit::event::{ElementState, MouseButton};
    use winit::keyboard::KeyCode;

    #[test]
    fn test_input_state_creation() {
        let input = InputState::new();
        assert!(!input.is_key_pressed(KeyCode::KeyA));
        assert!(!input.is_mouse_button_pressed(MouseButton::Left));
        assert_eq!(input.mouse_position(), Vec2::ZERO);
        assert_eq!(input.mouse_delta(), Vec2::ZERO);
        assert_eq!(input.mouse_wheel_delta(), 0.0);
    }

    #[test]
    fn test_keyboard_input() {
        let mut input = InputState::new();

        // Test key press
        input.handle_keyboard(KeyCode::Space, ElementState::Pressed);
        assert!(input.is_key_pressed(KeyCode::Space));
        assert!(input.is_key_just_pressed(KeyCode::Space));
        assert!(!input.is_key_just_released(KeyCode::Space));

        // Clear frame state
        input.clear_frame_state();
        assert!(input.is_key_pressed(KeyCode::Space));
        assert!(!input.is_key_just_pressed(KeyCode::Space));

        // Test key release
        input.handle_keyboard(KeyCode::Space, ElementState::Released);
        assert!(!input.is_key_pressed(KeyCode::Space));
        assert!(input.is_key_just_released(KeyCode::Space));

        // Clear frame state again
        input.clear_frame_state();
        assert!(!input.is_key_just_released(KeyCode::Space));
    }

    #[test]
    fn test_mouse_input() {
        let mut input = InputState::new();

        // Test mouse button
        input.handle_mouse_button(MouseButton::Left, ElementState::Pressed);
        assert!(input.is_mouse_button_pressed(MouseButton::Left));
        assert!(input.is_mouse_button_just_pressed(MouseButton::Left));

        input.clear_frame_state();
        assert!(input.is_mouse_button_pressed(MouseButton::Left));
        assert!(!input.is_mouse_button_just_pressed(MouseButton::Left));

        // Test mouse movement
        input.handle_mouse_motion(winit::dpi::PhysicalPosition::new(100.0, 200.0));
        assert_eq!(input.mouse_position(), Vec2::new(100.0, 200.0));
        assert_eq!(input.mouse_delta(), Vec2::new(100.0, 200.0));

        input.handle_mouse_motion(winit::dpi::PhysicalPosition::new(150.0, 250.0));
        assert_eq!(input.mouse_position(), Vec2::new(150.0, 250.0));
        assert_eq!(input.mouse_delta(), Vec2::new(50.0, 50.0));

        // Test mouse wheel
        input.handle_mouse_wheel(5.0);
        assert_eq!(input.mouse_wheel_delta(), 5.0);

        input.handle_mouse_wheel(3.0);
        assert_eq!(input.mouse_wheel_delta(), 8.0);

        input.clear_frame_state();
        assert_eq!(input.mouse_delta(), Vec2::ZERO);
        assert_eq!(input.mouse_wheel_delta(), 0.0);
    }

    #[test]
    fn test_multiple_keys() {
        let mut input = InputState::new();

        // Press multiple keys
        input.handle_keyboard(KeyCode::KeyW, ElementState::Pressed);
        input.handle_keyboard(KeyCode::KeyA, ElementState::Pressed);
        input.handle_keyboard(KeyCode::KeyS, ElementState::Pressed);
        input.handle_keyboard(KeyCode::KeyD, ElementState::Pressed);

        assert!(input.is_key_pressed(KeyCode::KeyW));
        assert!(input.is_key_pressed(KeyCode::KeyA));
        assert!(input.is_key_pressed(KeyCode::KeyS));
        assert!(input.is_key_pressed(KeyCode::KeyD));

        // Release some keys
        input.handle_keyboard(KeyCode::KeyW, ElementState::Released);
        input.handle_keyboard(KeyCode::KeyS, ElementState::Released);

        assert!(!input.is_key_pressed(KeyCode::KeyW));
        assert!(input.is_key_pressed(KeyCode::KeyA));
        assert!(!input.is_key_pressed(KeyCode::KeyS));
        assert!(input.is_key_pressed(KeyCode::KeyD));
    }
}
