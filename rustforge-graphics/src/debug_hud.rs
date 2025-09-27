//! Debug HUD system for displaying runtime information

use crate::text::TextRenderer;
use glam::{Vec2, Vec3};
use std::time::{Duration, Instant};

/// Debug information to display in HUD
#[derive(Debug, Clone)]
pub struct DebugInfo {
    pub fps: f32,
    pub frame_time_ms: f32,
    pub camera_position: Vec3,
    pub camera_rotation: glam::Quat,
    pub custom_info: Vec<(String, String)>, // Key-value pairs for custom debug info
}

/// FPS counter with smoothing
#[derive(Debug)]
pub struct FpsCounter {
    frame_count: u32,
    last_fps_update: Instant,
    current_fps: f32,
    frame_times: Vec<Duration>,
    max_samples: usize,
}

impl FpsCounter {
    pub fn new() -> Self {
        Self {
            frame_count: 0,
            last_fps_update: Instant::now(),
            current_fps: 0.0,
            frame_times: Vec::new(),
            max_samples: 60, // Average over 60 frames
        }
    }

    pub fn update(&mut self) -> f32 {
        let now = Instant::now();
        let frame_time = now.duration_since(self.last_fps_update);
        self.last_fps_update = now;

        // Store frame time for averaging
        self.frame_times.push(frame_time);
        if self.frame_times.len() > self.max_samples {
            self.frame_times.remove(0);
        }

        self.frame_count += 1;

        // Update FPS every few frames
        if self.frame_count % 10 == 0 && !self.frame_times.is_empty() {
            let avg_frame_time: Duration =
                self.frame_times.iter().sum::<Duration>() / self.frame_times.len() as u32;
            self.current_fps = 1.0 / avg_frame_time.as_secs_f32();
        }

        self.current_fps
    }

    pub fn fps(&self) -> f32 {
        self.current_fps
    }

    pub fn frame_time_ms(&self) -> f32 {
        if self.frame_times.is_empty() {
            0.0
        } else {
            let avg_frame_time: Duration =
                self.frame_times.iter().sum::<Duration>() / self.frame_times.len() as u32;
            avg_frame_time.as_secs_f32() * 1000.0
        }
    }
}

impl Default for FpsCounter {
    fn default() -> Self {
        Self::new()
    }
}

/// Debug HUD renderer with 2D text rendering
pub struct DebugHud {
    fps_counter: FpsCounter,
    show_hud: bool,
    text_position: Vec2,
    text_color: [f32; 4],
    text_scale: f32,
}

impl DebugHud {
    pub fn new() -> Self {
        Self {
            fps_counter: FpsCounter::new(),
            show_hud: true,
            text_position: Vec2::new(10.0, 10.0), // Top-left corner with margin
            text_color: [1.0, 1.0, 1.0, 1.0],     // White text
            text_scale: 1.0,
        }
    }

    pub fn update(&mut self, camera_position: Vec3, camera_rotation: glam::Quat) -> DebugInfo {
        let fps = self.fps_counter.update();
        let frame_time_ms = self.fps_counter.frame_time_ms();

        let debug_info = DebugInfo {
            fps,
            frame_time_ms,
            camera_position,
            camera_rotation,
            custom_info: Vec::new(),
        };

        debug_info
    }

    /// Add debug text to the text renderer for rendering
    pub fn add_debug_text(&self, text_renderer: &mut TextRenderer, info: &DebugInfo) {
        if !self.show_hud {
            return;
        }

        let (yaw, pitch, roll) = info.camera_rotation.to_euler(glam::EulerRot::YXZ);

        let mut text = format!(
            "FPS: {:.1} ({:.2}ms)\n\
             Camera: ({:.1}, {:.1}, {:.1})\n\
             Rotation: ({:.1}°, {:.1}°, {:.1}°)",
            info.fps,
            info.frame_time_ms,
            info.camera_position.x,
            info.camera_position.y,
            info.camera_position.z,
            yaw.to_degrees(),
            pitch.to_degrees(),
            roll.to_degrees()
        );

        for (key, value) in &info.custom_info {
            text.push_str(&format!("\n{}: {}", key, value));
        }

        text_renderer.add_text(&text, self.text_position, self.text_color, self.text_scale);
    }

    /// Get formatted debug text as string (for backward compatibility)
    pub fn format_debug_text(&self, info: &DebugInfo) -> String {
        if !self.show_hud {
            return String::new();
        }

        let (yaw, pitch, roll) = info.camera_rotation.to_euler(glam::EulerRot::YXZ);

        let mut text = format!(
            "FPS: {:.1} ({:.2}ms)\n\
             Camera: ({:.1}, {:.1}, {:.1})\n\
             Rotation: ({:.1}°, {:.1}°, {:.1}°)",
            info.fps,
            info.frame_time_ms,
            info.camera_position.x,
            info.camera_position.y,
            info.camera_position.z,
            yaw.to_degrees(),
            pitch.to_degrees(),
            roll.to_degrees()
        );

        for (key, value) in &info.custom_info {
            text.push_str(&format!("\n{}: {}", key, value));
        }

        text
    }

    pub fn add_custom_info(&mut self, debug_info: &mut DebugInfo, key: &str, value: &str) {
        debug_info
            .custom_info
            .push((key.to_string(), value.to_string()));
    }

    pub fn toggle_visibility(&mut self) {
        self.show_hud = !self.show_hud;
    }

    pub fn is_visible(&self) -> bool {
        self.show_hud
    }

    /// Set text rendering parameters
    pub fn set_text_position(&mut self, position: Vec2) {
        self.text_position = position;
    }

    pub fn set_text_color(&mut self, color: [f32; 4]) {
        self.text_color = color;
    }

    pub fn set_text_scale(&mut self, scale: f32) {
        self.text_scale = scale;
    }
}

impl Default for DebugHud {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_fps_counter_creation() {
        let fps_counter = FpsCounter::new();
        assert_eq!(fps_counter.fps(), 0.0);
        assert_eq!(fps_counter.frame_time_ms(), 0.0);
    }

    #[test]
    fn test_fps_counter_update() {
        let mut fps_counter = FpsCounter::new();

        // Simulate some frame updates
        for _ in 0..15 {
            thread::sleep(Duration::from_millis(16)); // ~60 FPS
            fps_counter.update();
        }

        // FPS should be roughly 60 (allowing for some variance)
        let fps = fps_counter.fps();
        assert!(
            fps > 50.0 && fps < 70.0,
            "FPS should be around 60, got {}",
            fps
        );
    }

    #[test]
    fn test_debug_hud_creation() {
        let hud = DebugHud::new();
        assert!(hud.is_visible());
    }

    #[test]
    fn test_debug_hud_toggle() {
        let mut hud = DebugHud::new();
        assert!(hud.is_visible());

        hud.toggle_visibility();
        assert!(!hud.is_visible());

        hud.toggle_visibility();
        assert!(hud.is_visible());
    }

    #[test]
    fn test_debug_info_custom_data() {
        let mut debug_info = DebugInfo {
            fps: 60.0,
            frame_time_ms: 16.67,
            camera_position: Vec3::ZERO,
            camera_rotation: glam::Quat::IDENTITY,
            custom_info: Vec::new(),
        };

        let mut hud = DebugHud::new();
        hud.add_custom_info(&mut debug_info, "Planets", "6");
        hud.add_custom_info(&mut debug_info, "Render Mode", "Instanced");

        assert_eq!(debug_info.custom_info.len(), 2);
        assert_eq!(
            debug_info.custom_info[0],
            ("Planets".to_string(), "6".to_string())
        );
        assert_eq!(
            debug_info.custom_info[1],
            ("Render Mode".to_string(), "Instanced".to_string())
        );
    }
}
