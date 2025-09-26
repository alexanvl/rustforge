//! Time management utilities

use std::time::{Duration, Instant};

/// Time tracking for the game loop
#[derive(Debug, Clone, Copy)]
pub struct Time {
    start_time: Instant,
    last_update: Instant,
    delta_time: Duration,
    total_time: Duration,
    fixed_timestep: Duration,
    accumulator: Duration,
}

impl Time {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            start_time: now,
            last_update: now,
            delta_time: Duration::ZERO,
            total_time: Duration::ZERO,
            fixed_timestep: Duration::from_secs_f64(1.0 / 60.0), // 60 FPS
            accumulator: Duration::ZERO,
        }
    }

    /// Update time tracking
    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta_time = now - self.last_update;
        self.last_update = now;
        self.total_time = now - self.start_time;
        self.accumulator += self.delta_time;
    }

    /// Get delta time in seconds
    pub fn delta_seconds(&self) -> f32 {
        self.delta_time.as_secs_f32()
    }

    /// Get delta time as Duration
    pub fn delta_time(&self) -> Duration {
        self.delta_time
    }

    /// Get total elapsed time in seconds
    pub fn elapsed_seconds(&self) -> f32 {
        self.total_time.as_secs_f32()
    }

    /// Get fixed timestep
    pub fn fixed_timestep(&self) -> Duration {
        self.fixed_timestep
    }

    /// Check if fixed update should run
    pub fn should_fixed_update(&mut self) -> bool {
        if self.accumulator >= self.fixed_timestep {
            self.accumulator -= self.fixed_timestep;
            true
        } else {
            false
        }
    }
}

impl Default for Time {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_time_creation() {
        let time = Time::new();
        assert_eq!(time.delta_seconds(), 0.0);
        assert_eq!(time.elapsed_seconds(), 0.0);
        assert_eq!(time.fixed_timestep(), Duration::from_secs_f64(1.0 / 60.0));
    }

    #[test]
    fn test_time_update() {
        let mut time = Time::new();

        // Sleep for a small duration
        thread::sleep(Duration::from_millis(50));

        time.update();

        // Delta time should be approximately 50ms
        assert!(time.delta_seconds() > 0.04);
        assert!(time.delta_seconds() < 0.06);

        // Total time should also be approximately 50ms
        assert!(time.elapsed_seconds() > 0.04);
        assert!(time.elapsed_seconds() < 0.06);
    }

    #[test]
    fn test_fixed_timestep() {
        let mut time = Time::new();
        let timestep = 1.0 / 60.0;

        // Initially should not need fixed update
        assert!(!time.should_fixed_update());

        // Simulate time passing for one fixed timestep
        time.accumulator = Duration::from_secs_f64(timestep);
        assert!(time.should_fixed_update());

        // After fixed update, accumulator should be empty
        assert!(!time.should_fixed_update());

        // Test accumulator less than fixed timestep
        time.accumulator = Duration::from_secs_f64(timestep * 0.5);
        assert!(!time.should_fixed_update());

        // Test accumulator exactly at fixed timestep
        time.accumulator = Duration::from_secs_f64(timestep);
        assert!(time.should_fixed_update());
        assert!(!time.should_fixed_update());

        // Test multiple timesteps accumulated
        time.accumulator = Duration::from_secs_f64(timestep * 2.5);

        // Should be able to do 2 fixed updates
        assert!(time.should_fixed_update());
        assert!(time.should_fixed_update());

        // But not a third (only 0.5 timesteps left)
        assert!(!time.should_fixed_update());

        // Accumulator should still have 0.5 timesteps
        assert!(time.accumulator.as_secs_f64() > timestep * 0.4);
        assert!(time.accumulator.as_secs_f64() < timestep * 0.6);
    }
}
