//! Solar System Demo - Realistic 3D solar system with orbital mechanics
//! This demonstrates proper instanced rendering, orbital physics, and lighting

use winit::{
    event::{Event, WindowEvent, ElementState, MouseButton, VirtualKeyCode as KeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
    dpi::LogicalSize,
};
use wgpu::util::DeviceExt;
use rustforge_core::prelude::*;
use rustforge_graphics::prelude::*;
use glam::{Vec3, Mat4, Quat};
use std::time::Instant;
use std::collections::HashSet;
use bytemuck::{Pod, Zeroable};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Uniforms {
    view: [[f32; 4]; 4],
    projection: [[f32; 4]; 4],
    light_position: [f32; 3],
    _padding1: f32,
    light_color: [f32; 3],
    _padding2: f32,
    camera_position: [f32; 3],
    _padding3: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct InstanceData {
    model: [[f32; 4]; 4],
    color: [f32; 3],
    emissive: f32,
}

struct Planet {
    position: Vec3,
    radius: f32,
    orbital_radius: f32,
    orbital_speed: f32,
    color: [f32; 3],
    emissive: bool,
}

struct SolarSystemDemo {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    num_indices: u32,
    camera: Camera,
    planets: Vec<Planet>,
    start_time: Instant,

    // Camera controls
    camera_speed: f32,
    mouse_sensitivity: f32,
    keys_pressed: HashSet<KeyCode>,
    mouse_pressed: bool,
    last_mouse_pos: (f64, f64),
}

impl SolarSystemDemo {
    async fn new(window: &Window) -> Result<Self> {
        // Create wgpu instance
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = unsafe { instance.create_surface(&window) }
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
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .map_err(|e| Error::Graphics(format!("Failed to create device: {}", e)))?;

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
        };
        surface.configure(&device, &config);

        // Create sphere geometry
        let (vertices, indices) = Self::create_sphere(32, 16);
        let num_indices = indices.len() as u32;

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        // Create uniform buffers
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Uniform Buffer"),
            size: std::mem::size_of::<Uniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create instance buffer (will be updated each frame)
        let max_instances = 10; // Support up to 10 planets
        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Instance Buffer"),
            size: (std::mem::size_of::<InstanceData>() * max_instances) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        // Create bind group layout (just for global uniforms)
        let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("uniform_bind_group_layout"),
        });

        // Create bind group (just for global uniforms)
        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("uniform_bind_group"),
        });

        // Create render pipeline
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&uniform_bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[
                    // Vertex buffer
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[
                            wgpu::VertexAttribute {
                                offset: 0,
                                shader_location: 0,
                                format: wgpu::VertexFormat::Float32x3,
                            },
                            wgpu::VertexAttribute {
                                offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                                shader_location: 1,
                                format: wgpu::VertexFormat::Float32x3,
                            },
                        ],
                    },
                    // Instance buffer
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<InstanceData>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &[
                            // Model matrix (4x4, takes 4 attribute slots)
                            wgpu::VertexAttribute {
                                offset: 0,
                                shader_location: 2,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            wgpu::VertexAttribute {
                                offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                                shader_location: 3,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            wgpu::VertexAttribute {
                                offset: (2 * std::mem::size_of::<[f32; 4]>()) as wgpu::BufferAddress,
                                shader_location: 4,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            wgpu::VertexAttribute {
                                offset: (3 * std::mem::size_of::<[f32; 4]>()) as wgpu::BufferAddress,
                                shader_location: 5,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            // Color
                            wgpu::VertexAttribute {
                                offset: (4 * std::mem::size_of::<[f32; 4]>()) as wgpu::BufferAddress,
                                shader_location: 6,
                                format: wgpu::VertexFormat::Float32x3,
                            },
                            // Emissive
                            wgpu::VertexAttribute {
                                offset: (4 * std::mem::size_of::<[f32; 4]>() + std::mem::size_of::<[f32; 3]>()) as wgpu::BufferAddress,
                                shader_location: 7,
                                format: wgpu::VertexFormat::Float32,
                            },
                        ],
                    },
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        // Create camera
        let mut camera = Camera::perspective(75.0, WIDTH as f32 / HEIGHT as f32, 0.1, 1000.0);
        camera.transform.position = Vec3::new(0.0, 15.0, 30.0); // High angle to see entire solar system

        // Create realistic solar system
        let planets = vec![
            // Sun (center, light source)
            Planet {
                position: Vec3::ZERO,
                radius: 2.0,
                orbital_radius: 0.0,
                orbital_speed: 0.0,
                color: [1.0, 0.9, 0.3], // Bright yellow sun
                emissive: true,
            },
            // Mercury
            Planet {
                position: Vec3::new(6.0, 0.0, 0.0),
                radius: 0.3,
                orbital_radius: 6.0,
                orbital_speed: 4.0,
                color: [0.7, 0.5, 0.4], // Gray-brown
                emissive: false,
            },
            // Venus
            Planet {
                position: Vec3::new(8.0, 0.0, 0.0),
                radius: 0.5,
                orbital_radius: 8.0,
                orbital_speed: 3.0,
                color: [1.0, 0.8, 0.2], // Yellow-orange
                emissive: false,
            },
            // Earth
            Planet {
                position: Vec3::new(10.0, 0.0, 0.0),
                radius: 0.5,
                orbital_radius: 10.0,
                orbital_speed: 2.0,
                color: [0.2, 0.4, 0.8], // Blue
                emissive: false,
            },
            // Mars
            Planet {
                position: Vec3::new(14.0, 0.0, 0.0),
                radius: 0.4,
                orbital_radius: 14.0,
                orbital_speed: 1.5,
                color: [0.8, 0.3, 0.2], // Red
                emissive: false,
            },
            // Jupiter
            Planet {
                position: Vec3::new(20.0, 0.0, 0.0),
                radius: 1.2,
                orbital_radius: 20.0,
                orbital_speed: 1.0,
                color: [0.9, 0.7, 0.4], // Orange-brown
                emissive: false,
            },
        ];

        Ok(Self {
            surface,
            device,
            queue,
            config,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            uniform_buffer,
            instance_buffer,
            uniform_bind_group,
            num_indices,
            camera,
            planets,
            start_time: Instant::now(),

            // Camera controls
            camera_speed: 10.0,
            mouse_sensitivity: 0.002,
            keys_pressed: HashSet::new(),
            mouse_pressed: false,
            last_mouse_pos: (0.0, 0.0),
        })
    }

    fn create_sphere(rings: u32, sectors: u32) -> (Vec<Vertex>, Vec<u32>) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let r = 1.0;
        let sector_step = 2.0 * std::f32::consts::PI / sectors as f32;
        let stack_step = std::f32::consts::PI / rings as f32;

        for i in 0..=rings {
            let stack_angle = std::f32::consts::PI / 2.0 - i as f32 * stack_step;
            let xy = r * stack_angle.cos();
            let z = r * stack_angle.sin();

            for j in 0..=sectors {
                let sector_angle = j as f32 * sector_step;
                let x = xy * sector_angle.cos();
                let y = xy * sector_angle.sin();

                vertices.push(Vertex {
                    position: [x, y, z],
                    normal: [x, y, z], // For a unit sphere, normal = position
                });
            }
        }

        for i in 0..rings {
            let mut k1 = i * (sectors + 1);
            let mut k2 = k1 + sectors + 1;

            for _j in 0..sectors {
                if i != 0 {
                    indices.extend_from_slice(&[k1, k2, k1 + 1]);
                }
                if i != (rings - 1) {
                    indices.extend_from_slice(&[k1 + 1, k2, k2 + 1]);
                }
                k1 += 1;
                k2 += 1;
            }
        }

        (vertices, indices)
    }

    fn update(&mut self) {
        let elapsed = self.start_time.elapsed().as_secs_f32();

        // Update planet positions with orbital mechanics
        for planet in &mut self.planets {
            if planet.orbital_radius > 0.0 {
                // Calculate orbital position
                let angle = elapsed * planet.orbital_speed * 0.1; // Slow down for visibility
                planet.position = Vec3::new(
                    planet.orbital_radius * angle.cos(),
                    0.0,
                    planet.orbital_radius * angle.sin(),
                );
            }
        }

        // Handle camera movement
        self.handle_camera_movement(1.0 / 60.0); // Assume 60 FPS
    }

    fn handle_camera_movement(&mut self, delta_time: f32) {
        let speed = if self.keys_pressed.contains(&KeyCode::LShift) ||
                       self.keys_pressed.contains(&KeyCode::RShift) {
            self.camera_speed * 3.0 // Boost speed with shift
        } else {
            self.camera_speed
        };

        // Calculate movement direction
        let mut movement = Vec3::ZERO;

        if self.keys_pressed.contains(&KeyCode::W) {
            movement += self.camera.transform.forward();
        }
        if self.keys_pressed.contains(&KeyCode::S) {
            movement -= self.camera.transform.forward();
        }
        if self.keys_pressed.contains(&KeyCode::A) {
            movement -= self.camera.transform.right();
        }
        if self.keys_pressed.contains(&KeyCode::D) {
            movement += self.camera.transform.right();
        }
        if self.keys_pressed.contains(&KeyCode::Space) {
            movement += Vec3::Y; // Move up
        }
        if self.keys_pressed.contains(&KeyCode::LControl) ||
           self.keys_pressed.contains(&KeyCode::C) {
            movement -= Vec3::Y; // Move down
        }

        // Apply movement
        if movement.length_squared() > 0.0 {
            movement = movement.normalize();
            self.camera.transform.position += movement * speed * delta_time;
        }

        // Handle mouse look (simple version for now)
        if self.mouse_pressed {
            // This will be implemented when we get the mouse delta working
        }
    }


    fn render(&mut self) -> Result<()> {
        let output = self.surface.get_current_texture()
            .map_err(|e| Error::Graphics(format!("Failed to get current texture: {}", e)))?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        // Update global uniforms
        let view_matrix = self.camera.view_matrix();
        let projection_matrix = self.camera.projection_matrix();

        let uniforms = Uniforms {
            view: view_matrix.to_cols_array_2d(),
            projection: projection_matrix.to_cols_array_2d(),
            light_position: [0.0, 0.0, 0.0], // Sun at center
            _padding1: 0.0,
            light_color: [1.0, 0.9, 0.7],
            _padding2: 0.0,
            camera_position: self.camera.transform.position.to_array(),
            _padding3: 0.0,
        };

        // Prepare instance data for all planets
        let mut instance_data = Vec::new();
        for planet in &self.planets {
            let elapsed = self.start_time.elapsed().as_secs_f32();
            let rotation = Quat::from_rotation_y(elapsed * 0.5);
            let model_matrix = Mat4::from_scale_rotation_translation(
                Vec3::splat(planet.radius),
                rotation,
                planet.position,
            );

            instance_data.push(InstanceData {
                model: model_matrix.to_cols_array_2d(),
                color: planet.color,
                emissive: if planet.emissive { 1.0 } else { 0.0 },
            });
        }

        // Write all instance data at once before render pass
        self.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
        self.queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&instance_data));

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.02,
                            g: 0.02,
                            b: 0.05,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);

            // Render all planets in one instanced draw call
            render_pass.draw_indexed(0..self.num_indices, 0, 0..self.planets.len() as u32);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);

        }
    }

    async fn run(mut self, event_loop: EventLoop<()>) -> ! {
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent { ref event, .. } => {
                    match event {
                        WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit;
                        }
                        WindowEvent::Resized(physical_size) => {
                            self.resize(*physical_size);
                        }
                        WindowEvent::KeyboardInput { input, .. } => {
                            if let Some(key) = input.virtual_keycode {
                                match input.state {
                                    ElementState::Pressed => {
                                        self.keys_pressed.insert(key);
                                        // ESC to quit
                                        if key == KeyCode::Escape {
                                            *control_flow = ControlFlow::Exit;
                                        }
                                    }
                                    ElementState::Released => {
                                        self.keys_pressed.remove(&key);
                                    }
                                }
                            }
                        }
                        WindowEvent::MouseInput { button, state, .. } => {
                            if *button == MouseButton::Left {
                                self.mouse_pressed = *state == ElementState::Pressed;
                            }
                        }
                        WindowEvent::CursorMoved { position, .. } => {
                            // Simple mouse delta calculation
                            let delta_x = position.x - self.last_mouse_pos.0;
                            let delta_y = position.y - self.last_mouse_pos.1;

                            if self.mouse_pressed {
                                let yaw = -delta_x as f32 * self.mouse_sensitivity;
                                let pitch = -delta_y as f32 * self.mouse_sensitivity;

                                // Apply yaw (around Y axis)
                                let yaw_rotation = Quat::from_rotation_y(yaw);
                                self.camera.transform.rotation = yaw_rotation * self.camera.transform.rotation;

                                // Apply pitch (around local X axis)
                                let right = self.camera.transform.right();
                                let pitch_rotation = Quat::from_axis_angle(right, pitch);
                                self.camera.transform.rotation = pitch_rotation * self.camera.transform.rotation;
                            }

                            self.last_mouse_pos = (position.x, position.y);
                        }
                        _ => {}
                    }
                }
                Event::MainEventsCleared => {
                    self.update();
                    if let Err(e) = self.render() {
                        eprintln!("Render error: {}", e);
                    }
                }
                _ => {}
            }
        })
    }
}

fn main() -> Result<()> {
    env_logger::init();

    println!("🌟 Starting Solar System Demo...");
    println!("This demo shows:");
    println!("  ☀️  Sun as central light source");
    println!("  🪐 Multiple planets orbiting the sun");
    println!("  💡 Realistic lighting calculations");
    println!("  🎮 Free-floating camera controls");
    println!();
    println!("Controls:");
    println!("  WASD - Move camera");
    println!("  Space - Move up");
    println!("  C/Ctrl - Move down");
    println!("  Shift - Move faster");
    println!("  Left Mouse + Drag - Look around");
    println!("  ESC - Exit");

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("🌟 RustForge Solar System Demo")
        .with_inner_size(LogicalSize::new(WIDTH, HEIGHT))
        .build(&event_loop)
        .map_err(|e| Error::Init(format!("Failed to create window: {}", e)))?;

    let demo = pollster::block_on(SolarSystemDemo::new(&window))?;
    pollster::block_on(demo.run(event_loop));
}
