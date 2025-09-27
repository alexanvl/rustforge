//! Solar System Demo - Using the new app framework
//!
//! This demonstrates instanced rendering and orbital mechanics with minimal boilerplate.

use rustforge_app::prelude::*;
use glam::{Vec3, Mat4};
use bytemuck::{Pod, Zeroable};
use std::time::Instant;

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
    // Models
    sphere: Model,
    planets: Vec<Planet>,

    // Camera
    camera_controller: CameraController,

    // Render resources
    render_pipeline: wgpu::RenderPipeline,
    uniform_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,

    // Timing
    start_time: Instant,

    // Debug
    show_orbits: bool,
    paused: bool,
    speed_multiplier: f32,
}

impl App for SolarSystemDemo {
    fn init(ctx: &mut Context) -> Result<Self> {
        log::info!("Initializing Solar System Demo with new app framework");

        // Create sphere model
        let sphere = Model::sphere(&ctx.device, 32, 16);

        // Create planets
        let mut planets = Vec::new();

        // Sun
        planets.push(Planet {
            position: Vec3::ZERO,
            radius: 2.0,
            orbital_radius: 0.0,
            orbital_speed: 0.0,
            color: [1.0, 0.9, 0.0],
            emissive: true,
        });

        // Mercury
        planets.push(Planet {
            position: Vec3::new(5.0, 0.0, 0.0),
            radius: 0.4,
            orbital_radius: 5.0,
            orbital_speed: 4.0,
            color: [0.7, 0.6, 0.5],
            emissive: false,
        });

        // Venus
        planets.push(Planet {
            position: Vec3::new(8.0, 0.0, 0.0),
            radius: 0.9,
            orbital_radius: 8.0,
            orbital_speed: 3.0,
            color: [0.9, 0.7, 0.5],
            emissive: false,
        });

        // Earth
        planets.push(Planet {
            position: Vec3::new(11.0, 0.0, 0.0),
            radius: 1.0,
            orbital_radius: 11.0,
            orbital_speed: 2.0,
            color: [0.2, 0.5, 0.8],
            emissive: false,
        });

        // Mars
        planets.push(Planet {
            position: Vec3::new(15.0, 0.0, 0.0),
            radius: 0.5,
            orbital_radius: 15.0,
            orbital_speed: 1.5,
            color: [0.8, 0.4, 0.2],
            emissive: false,
        });

        // Create uniform buffer
        let uniform_buffer = ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Uniform Buffer"),
            size: std::mem::size_of::<Uniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create instance buffer
        let instance_buffer = ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Instance Buffer"),
            size: (std::mem::size_of::<InstanceData>() * planets.len()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create shader
        let shader = ctx.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Solar System Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("solar_system.wgsl").into()),
        });

        // Create bind group layout
        let bind_group_layout = ctx.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
            label: Some("bind_group_layout"),
        });

        let bind_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("bind_group"),
        });

        // Create pipeline
        let pipeline_layout = ctx.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = ctx.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[
                    PositionNormal::desc(),
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<InstanceData>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &[
                            // Model matrix (4x4)
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
                                offset: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                                shader_location: 4,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            wgpu::VertexAttribute {
                                offset: std::mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                                shader_location: 5,
                                format: wgpu::VertexFormat::Float32x4,
                            },
                            // Color
                            wgpu::VertexAttribute {
                                offset: std::mem::size_of::<[f32; 16]>() as wgpu::BufferAddress,
                                shader_location: 6,
                                format: wgpu::VertexFormat::Float32x3,
                            },
                            // Emissive
                            wgpu::VertexAttribute {
                                offset: std::mem::size_of::<[f32; 19]>() as wgpu::BufferAddress,
                                shader_location: 7,
                                format: wgpu::VertexFormat::Float32,
                            },
                        ],
                    },
                ],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Bgra8UnormSrgb,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
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
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        // Set initial camera position looking at the sun
        ctx.camera.transform.position = Vec3::new(20.0, 10.0, 20.0);
        // Calculate direction to look at origin
        let direction = (Vec3::ZERO - ctx.camera.transform.position).normalize();
        ctx.camera.transform.rotation = glam::Quat::from_rotation_arc(Vec3::NEG_Z, direction);

        Ok(Self {
            sphere,
            planets,
            camera_controller: CameraController::new(10.0, 0.005),
            render_pipeline,
            uniform_buffer,
            instance_buffer,
            bind_group,
            start_time: Instant::now(),
            show_orbits: false,
            paused: false,
            speed_multiplier: 1.0,
        })
    }

    fn update(&mut self, ctx: &mut Context, dt: f32) {
        // Update camera
        self.camera_controller.update(&mut ctx.camera, dt);

        // Update planet positions
        let elapsed = if self.paused {
            (self.start_time.elapsed().as_secs_f32() - dt) * self.speed_multiplier
        } else {
            self.start_time.elapsed().as_secs_f32() * self.speed_multiplier
        };

        let instances: Vec<InstanceData> = self.planets.iter_mut().map(|planet| {
            if planet.orbital_radius > 0.0 {
                let angle = elapsed * planet.orbital_speed;
                planet.position = Vec3::new(
                    planet.orbital_radius * angle.cos(),
                    0.0,
                    planet.orbital_radius * angle.sin(),
                );
            }

            let scale = Mat4::from_scale(Vec3::splat(planet.radius));
            let translation = Mat4::from_translation(planet.position);
            let model = translation * scale;

            InstanceData {
                model: model.to_cols_array_2d(),
                color: planet.color,
                emissive: if planet.emissive { 1.0 } else { 0.0 },
            }
        }).collect();

        // Update instance buffer
        ctx.queue.write_buffer(&self.instance_buffer, 0, bytemuck::cast_slice(&instances));

        // Update uniforms
        let uniforms = Uniforms {
            view: ctx.camera.view_matrix().to_cols_array_2d(),
            projection: ctx.camera.projection_matrix().to_cols_array_2d(),
            light_position: [0.0, 0.0, 0.0], // Sun position
            _padding1: 0.0,
            light_color: [1.0, 1.0, 0.9],
            _padding2: 0.0,
            camera_position: ctx.camera.transform.position.to_array(),
            _padding3: 0.0,
        };

        ctx.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
    }

    fn render(&mut self, ctx: &mut RenderContext) -> Result<()> {
        let mut render_pass = ctx.begin_render_pass("Main Pass");

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));

        // Set vertex buffers and draw with instancing
        self.sphere.set_buffers(&mut render_pass);
        render_pass.draw_indexed(0..self.sphere.num_indices, 0, 0..self.planets.len() as u32);

        Ok(())
    }

    fn handle_event(&mut self, _ctx: &mut Context, event: &WindowEvent) -> bool {
        use rustforge_app::prelude::{KeyCode, PhysicalKey, ElementState};

        if let WindowEvent::KeyboardInput { event, .. } = event {
            if event.state == ElementState::Pressed {
                if let PhysicalKey::Code(keycode) = event.physical_key {
                    match keycode {
                        KeyCode::Space => {
                            self.paused = !self.paused;
                            log::info!("Simulation {}", if self.paused { "paused" } else { "resumed" });
                            return true;
                        }
                        KeyCode::KeyO => {
                            self.show_orbits = !self.show_orbits;
                            log::info!("Orbits {}", if self.show_orbits { "shown" } else { "hidden" });
                            return true;
                        }
                        KeyCode::BracketLeft => {
                            self.speed_multiplier = (self.speed_multiplier * 0.5).max(0.1);
                            log::info!("Speed: {}x", self.speed_multiplier);
                            return true;
                        }
                        KeyCode::BracketRight => {
                            self.speed_multiplier = (self.speed_multiplier * 2.0).min(10.0);
                            log::info!("Speed: {}x", self.speed_multiplier);
                            return true;
                        }
                        _ => {}
                    }
                }
            }
        }

        self.camera_controller.handle_event(event)
    }

    fn config() -> AppConfig {
        AppConfig {
            title: "RustForge Solar System Demo - New Framework".to_string(),
            width: 1024,
            height: 768,
            ..Default::default()
        }
    }
}

fn main() -> Result<()> {
    run::<SolarSystemDemo>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uniforms_size() {
        assert_eq!(std::mem::size_of::<Uniforms>(), 160);
        assert_eq!(std::mem::size_of::<InstanceData>(), 80);
    }
}
