//! Simple demo to test the application framework

use crate::prelude::*;
use glam::Vec3;

pub struct SimpleDemo {
    camera_controller: CameraController,
    time: f32,
}

impl App for SimpleDemo {
    fn init(_ctx: &mut Context) -> Result<Self> {
        log::info!("Initializing Simple Demo");

        Ok(Self {
            camera_controller: CameraController::default(),
            time: 0.0,
        })
    }

    fn update(&mut self, ctx: &mut Context, dt: f32) {
        self.time += dt;
        self.camera_controller.update(&mut ctx.camera, dt);

        // Simple animation - rotate camera position
        let radius = 5.0;
        ctx.camera.transform.position = Vec3::new(
            radius * self.time.cos(),
            2.0,
            radius * self.time.sin(),
        );

        // Look at origin
        // Look at origin
        let direction = (Vec3::ZERO - ctx.camera.transform.position).normalize();
        ctx.camera.transform.rotation = glam::Quat::from_rotation_arc(Vec3::NEG_Z, direction);
    }

    fn render(&mut self, ctx: &mut RenderContext) -> Result<()> {
        // For now, just clear the screen
        let _render_pass = ctx.begin_render_pass("Simple Demo Pass");

        Ok(())
    }

    fn handle_event(&mut self, _ctx: &mut Context, event: &WindowEvent) -> bool {
        self.camera_controller.handle_event(event)
    }

    fn config() -> AppConfig {
        AppConfig {
            title: "RustForge Simple Demo".to_string(),
            width: 800,
            height: 600,
            ..Default::default()
        }
    }
}
