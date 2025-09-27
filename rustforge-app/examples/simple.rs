//! Simple example using the application framework

use rustforge_app::prelude::*;

struct SimpleApp {
    time: f32,
}

impl App for SimpleApp {
    fn init(_ctx: &mut Context) -> Result<Self> {
        log::info!("Simple app initialized!");
        Ok(Self { time: 0.0 })
    }

    fn update(&mut self, _ctx: &mut Context, dt: f32) {
        self.time += dt;
        if self.time > 1.0 {
            log::info!("Running for {:.1} seconds", self.time);
            self.time -= 1.0;
        }
    }

    fn render(&mut self, ctx: &mut RenderContext) -> Result<()> {
        // Just clear the screen
        let _pass = ctx.begin_render_pass("Main Pass");
        Ok(())
    }

    fn config() -> AppConfig {
        AppConfig {
            title: "Simple RustForge App".to_string(),
            width: 640,
            height: 480,
            ..Default::default()
        }
    }
}

fn main() -> Result<()> {
    rustforge_app::prelude::run::<SimpleApp>()
}
