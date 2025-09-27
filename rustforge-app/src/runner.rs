//! Application runner - handles window creation, event loop, and rendering

use crate::app::App;
use crate::context::{Context, RenderContext};
use rustforge_core::prelude::*;
use std::sync::Arc;
use std::time::Instant;
use winit::{
    event::{Event, WindowEvent, ElementState},
    event_loop::EventLoop,
    window::Window,
    dpi::LogicalSize,
    keyboard::{PhysicalKey, KeyCode},
};

/// Run an application
pub fn run<T: App + 'static>() -> Result<()> {
    env_logger::init();

    let config = T::config();

    log::info!("Starting {}", config.title);

    let event_loop = EventLoop::new()
        .map_err(|e| Error::Init(format!("Failed to create event loop: {:?}", e)))?;

    let window = Arc::new(event_loop.create_window(
        Window::default_attributes()
            .with_title(&config.title)
            .with_inner_size(LogicalSize::new(config.width, config.height))
            .with_resizable(config.resizable)
    ).map_err(|e| Error::Init(format!("Failed to create window: {}", e)))?);

    // Run the application
    pollster::block_on(run_async::<T>(event_loop, window))
}

async fn run_async<T: App + 'static>(event_loop: EventLoop<()>, window: Arc<Window>) -> Result<()> {
    let mut size = window.inner_size();

    // Initialize WGPU
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });

    let surface = instance.create_surface(window.clone())
        .map_err(|e| Error::Graphics(format!("Failed to create surface: {}", e)))?;

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .ok_or_else(|| Error::Graphics("Failed to find suitable adapter".into()))?;

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: None,
                memory_hints: Default::default(),
            },
            None,
        )
        .await
        .map_err(|e| Error::Graphics(format!("Failed to create device: {}", e)))?;

    let device = Arc::new(device);
    let queue = Arc::new(queue);

    let surface_caps = surface.get_capabilities(&adapter);
    let surface_format = surface_caps.formats.iter()
        .copied()
        .find(|f| f.is_srgb())
        .unwrap_or(surface_caps.formats[0]);

    let mut config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode: if T::config().vsync {
            wgpu::PresentMode::AutoVsync
        } else {
            wgpu::PresentMode::AutoNoVsync
        },
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };
    surface.configure(&device, &config);

    // Create depth texture
    let (mut depth_texture, mut depth_view) = create_depth_texture(&device, &config);

    // Create context and app
    let mut context = Context::new(
        device.clone(),
        queue.clone(),
        (size.width, size.height),
    );

    let mut app = T::init(&mut context)?;
    let mut last_frame = Instant::now();

    // Request initial redraw
    window.request_redraw();

    event_loop.run(move |event, event_loop| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                // Update input state
                context.input.handle_event(event);

                // Let app handle event
                if !app.handle_event(&mut context, event) {
                    match event {
                        WindowEvent::CloseRequested => event_loop.exit(),
                        WindowEvent::KeyboardInput {
                            event,
                            ..
                        } if event.state == ElementState::Pressed
                            && event.physical_key == PhysicalKey::Code(KeyCode::Escape) => {
                            event_loop.exit();
                        }
                        WindowEvent::Resized(physical_size) => {
                            size = *physical_size;
                            if size.width > 0 && size.height > 0 {
                                config.width = size.width;
                                config.height = size.height;
                                surface.configure(&device, &config);

                                // Recreate depth texture
                                let (new_depth_texture, new_depth_view) = create_depth_texture(&device, &config);
                                depth_texture = new_depth_texture;
                                depth_view = new_depth_view;

                                // Update context
                                context.window_size = (size.width, size.height);
                                context.camera.aspect_ratio = size.width as f32 / size.height as f32;

                                // Notify app
                                app.on_resize(&mut context, size.width, size.height);
                            }
                        }
                        WindowEvent::RedrawRequested => {
                            let now = Instant::now();
                            let dt = (now - last_frame).as_secs_f32();
                            last_frame = now;

                            app.update(&mut context, dt);

                            match render(&surface, &device, &queue, &depth_view, &context, &mut app) {
                                Ok(_) => {}
                                Err(wgpu::SurfaceError::Lost) => {
                                    log::warn!("Surface lost, reconfiguring...");
                                    config.width = size.width;
                                    config.height = size.height;
                                    surface.configure(&device, &config);
                                }
                                Err(wgpu::SurfaceError::OutOfMemory) => {
                                    log::error!("Out of memory!");
                                    event_loop.exit();
                                }
                                Err(e) => {
                                    log::error!("Render error: {:?}", e);
                                }
                            }

                            window.request_redraw();
                        }
                        _ => {}
                    }
                }
            }
            Event::AboutToWait => {
                // Continuously request redraws
                window.request_redraw();
            }
            _ => {}
        }
    })
    .map_err(|e| Error::Init(format!("Event loop error: {:?}", e)))?;

    Ok(())
}

fn render<T: App>(
    surface: &wgpu::Surface,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    depth_view: &wgpu::TextureView,
    context: &Context,
    app: &mut T,
) -> std::result::Result<(), wgpu::SurfaceError> {
    let output = surface.get_current_texture()?;
    let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
    });

    {
        let mut render_context = RenderContext {
            encoder: &mut encoder,
            view: &view,
            depth_view,
            camera: &context.camera,
            device,
            queue,
        };

        if let Err(e) = app.render(&mut render_context) {
            log::error!("App render error: {:?}", e);
        }
    }

    queue.submit(std::iter::once(encoder.finish()));
    output.present();

    Ok(())
}

fn create_depth_texture(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> (wgpu::Texture, wgpu::TextureView) {
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Depth Texture"),
        size: wgpu::Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });

    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    (texture, view)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_depth_texture() {
        // This test would require a device, which requires async setup
        // For now, we just ensure the function compiles
    }
}