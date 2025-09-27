//! Text Rendering Demo
//! This example demonstrates the 2D text rendering functionality
//! by creating a window and rendering actual text using wgpu.

use rustforge_core::prelude::*;
use rustforge_graphics::prelude::*;
use std::sync::Arc;
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::KeyCode,
    raw_window_handle::{HasDisplayHandle, HasWindowHandle},
};

/// Text rendering demonstration
/// This creates a window and renders 2D text using the TextRenderer
fn main() -> Result<()> {
    env_logger::init();

    let event_loop = EventLoop::new()
        .map_err(|e| Error::Graphics(format!("Failed to create event loop: {:?}", e)))?;
    let window = event_loop
        .create_window(
            winit::window::WindowAttributes::default()
                .with_title("RustForge Text Rendering Demo")
                .with_inner_size(LogicalSize::new(800, 600)),
        )
        .map_err(|e| Error::Graphics(format!("Failed to create window: {:?}", e)))?;

    // Create wgpu instance
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());

    // Create surface - try the simple approach first
    let surface = match instance.create_surface(&window) {
        Ok(surface) => surface,
        Err(_) => {
            // Fallback: try unsafe surface creation
            println!("Using fallback surface creation...");
            let raw_window_handle = window
                .window_handle()
                .map_err(|e| Error::Graphics(format!("Failed to get window handle: {:?}", e)))?;
            let raw_display_handle = window
                .display_handle()
                .map_err(|e| Error::Graphics(format!("Failed to get display handle: {:?}", e)))?;
            unsafe {
                instance.create_surface_unsafe(wgpu::SurfaceTargetUnsafe::RawHandle {
                    raw_display_handle: raw_display_handle.as_raw(),
                    raw_window_handle: raw_window_handle.as_raw(),
                })
            }
            .map_err(|e| Error::Graphics(format!("Failed to create surface: {}", e)))?
        }
    };

    // Request adapter
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
    }))
    .ok_or_else(|| Error::Graphics("Failed to find suitable adapter".into()))?;

    // Create device and queue
    let (device, queue) = pollster::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            label: None,
            memory_hints: Default::default(),
        },
        None,
    ))
    .map_err(|e| Error::Graphics(format!("Failed to create device: {}", e)))?;

    let device = Arc::new(device);
    let queue = Arc::new(queue);

    // Configure surface
    let surface_caps = surface.get_capabilities(&adapter);
    let surface_format = surface_caps.formats[0];
    let size = window.inner_size();

    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode: surface_caps.present_modes[0],
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };
    surface.configure(&device, &config);

    // Load font data
    let font_data = include_bytes!("../assets/Geneva.ttf");

    // Create text renderer with font
    let mut text_renderer = TextRenderer::new_with_font(&device, &queue, config.format, font_data)?;

    // Create debug HUD
    let mut debug_hud = DebugHud::new();
    debug_hud.set_text_position(glam::Vec2::new(20.0, 20.0));
    debug_hud.set_text_color([0.9, 0.9, 1.0, 1.0]);

    // Demo text that doesn't require camera/physics data
    let mut frame_count = 0;
    let start_time = std::time::Instant::now();

    let _ = event_loop.run(move |event, event_loop_window_target| {
        event_loop_window_target.set_control_flow(ControlFlow::Poll);

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    event_loop_window_target.exit();
                }
                WindowEvent::KeyboardInput { event, .. } => {
                    if let winit::keyboard::PhysicalKey::Code(KeyCode::Escape) = event.physical_key
                    {
                        if event.state == ElementState::Pressed {
                            event_loop_window_target.exit();
                        }
                    }
                }
                WindowEvent::Resized(size) => {
                    // Handle resize
                    let config = wgpu::SurfaceConfiguration {
                        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                        format: surface_format,
                        width: size.width,
                        height: size.height,
                        present_mode: surface_caps.present_modes[0],
                        alpha_mode: surface_caps.alpha_modes[0],
                        view_formats: vec![],
                        desired_maximum_frame_latency: 2,
                    };
                    surface.configure(&device, &config);
                    text_renderer.resize(size.width, size.height);
                }
                _ => {}
            },
            Event::NewEvents(_) => {
                frame_count += 1;
                let elapsed = start_time.elapsed();
                let fps = frame_count as f32 / elapsed.as_secs_f32();

                // Create demo debug info
                let debug_info = DebugInfo {
                    fps,
                    frame_time_ms: 1000.0 / fps,
                    camera_position: glam::Vec3::new(0.0, 0.0, -5.0),
                    camera_rotation: glam::Quat::IDENTITY,
                    custom_info: vec![
                        ("Frame Count".to_string(), frame_count.to_string()),
                        (
                            "Uptime".to_string(),
                            format!("{:.1}s", elapsed.as_secs_f32()),
                        ),
                        ("Status".to_string(), "Rendering Text".to_string()),
                    ],
                };

                // Clear and add debug text
                text_renderer.clear();
                // For now, just add some simple text
                text_renderer.add_text(
                    &format!("FPS: {:.1}\nFrame: {}", fps, frame_count),
                    glam::Vec2::new(20.0, 20.0),
                    [0.9, 0.9, 1.0, 1.0],
                    1.0,
                );

                // Debug: print what we're rendering
                if frame_count % 60 == 0 {
                    // Every second at 60 FPS
                    println!(
                        "Frame {}: FPS: {:.1}, Text queued: {}",
                        frame_count,
                        fps,
                        debug_info.fps > 0.0
                    );
                }

                // Render frame
                let output = surface
                    .get_current_texture()
                    .map_err(|e| Error::Graphics(format!("Failed to get surface texture: {}", e)))
                    .unwrap();

                let view = output
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Text Demo Encoder"),
                });

                {
                    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Text Demo Render Pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color {
                                    r: 0.1,
                                    g: 0.1,
                                    b: 0.2,
                                    a: 1.0,
                                }),
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        depth_stencil_attachment: None,
                        occlusion_query_set: None,
                        timestamp_writes: None,
                    });

                    // Render text
                    text_renderer.render(&mut render_pass).unwrap();
                }

                queue.submit(std::iter::once(encoder.finish()));
                output.present();
            }
            _ => {}
        }
    });

    Ok(())
}
