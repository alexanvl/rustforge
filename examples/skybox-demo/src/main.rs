//! Skybox Demo - Complete implementation using the new app framework
//!
//! This demonstrates how the new rustforge-app framework dramatically reduces
//! boilerplate while still providing full functionality.

use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec3};
use rustforge_app::prelude::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct SkyboxUniforms {
    view_proj: [[f32; 4]; 4],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct ModelUniforms {
    model: [[f32; 4]; 4],
    view: [[f32; 4]; 4],
    projection: [[f32; 4]; 4],
    light_position: [f32; 3],
    _padding1: f32,
    camera_position: [f32; 3],
    _padding2: f32,
}

struct SkyboxDemo {
    // Models
    skybox: Skybox,
    sphere: Model,

    // Camera
    camera_controller: CameraController,

    // Render resources
    skybox_uniform_buffer: wgpu::Buffer,
    skybox_bind_group: wgpu::BindGroup,
    skybox_texture_bind_group: wgpu::BindGroup,

    model_uniform_buffer: wgpu::Buffer,
    model_bind_group: wgpu::BindGroup,
    model_pipeline: wgpu::RenderPipeline,
}

impl App for SkyboxDemo {
    fn init(ctx: &mut Context) -> Result<Self> {
        log::info!("Initializing Skybox Demo with new app framework");

        // Create skybox
        let skybox = Skybox::from_cubemap(
            &ctx.device,
            &ctx.queue,
            wgpu::TextureFormat::Bgra8UnormSrgb,
            "cubemap",
        )?;

        // Create sphere
        let sphere = Model::sphere(&ctx.device, 32, 16);

        // Create skybox uniform buffer
        let skybox_uniform_buffer = ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Skybox Uniform Buffer"),
            size: std::mem::size_of::<SkyboxUniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create skybox bind group
        let skybox_bind_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: skybox.uniform_bind_group_layout(),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: skybox_uniform_buffer.as_entire_binding(),
            }],
            label: Some("skybox_bind_group"),
        });

        let skybox_texture_bind_group = skybox.create_texture_bind_group(&ctx.device);

        // Create model uniform buffer
        let model_uniform_buffer = ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Model Uniform Buffer"),
            size: std::mem::size_of::<ModelUniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create model pipeline (simplified for now)
        let shader = ctx
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Model Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("model.wgsl").into()),
            });

        let bind_group_layout =
            ctx.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
                    label: Some("model_bind_group_layout"),
                });

        let model_bind_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: model_uniform_buffer.as_entire_binding(),
            }],
            label: Some("model_bind_group"),
        });

        let pipeline_layout = ctx
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Model Pipeline Layout"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });

        let model_pipeline = ctx
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Model Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    buffers: &[PositionNormal::desc()],
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

        // Set initial camera position
        ctx.camera.transform.position = Vec3::new(0.0, 0.0, 5.0);

        Ok(Self {
            skybox,
            sphere,
            camera_controller: CameraController::default(),
            skybox_uniform_buffer,
            skybox_bind_group,
            skybox_texture_bind_group,
            model_uniform_buffer,
            model_bind_group,
            model_pipeline,
        })
    }

    fn update(&mut self, ctx: &mut Context, dt: f32) {
        // Update camera
        self.camera_controller.update(&mut ctx.camera, dt);

        // Update skybox uniforms
        let view_matrix = Mat4::from_rotation_translation(
            ctx.camera.transform.rotation,
            Vec3::ZERO, // No translation for skybox
        );
        let view_proj = ctx.camera.projection_matrix() * view_matrix;

        let skybox_uniforms = SkyboxUniforms {
            view_proj: view_proj.to_cols_array_2d(),
        };

        ctx.queue.write_buffer(
            &self.skybox_uniform_buffer,
            0,
            bytemuck::cast_slice(&[skybox_uniforms]),
        );

        // Update model uniforms
        let model_uniforms = ModelUniforms {
            model: Mat4::IDENTITY.to_cols_array_2d(),
            view: ctx.camera.view_matrix().to_cols_array_2d(),
            projection: ctx.camera.projection_matrix().to_cols_array_2d(),
            light_position: [2.0, 5.0, 2.0],
            _padding1: 0.0,
            camera_position: ctx.camera.transform.position.to_array(),
            _padding2: 0.0,
        };

        ctx.queue.write_buffer(
            &self.model_uniform_buffer,
            0,
            bytemuck::cast_slice(&[model_uniforms]),
        );
    }

    fn render(&mut self, ctx: &mut RenderContext) -> Result<()> {
        let mut render_pass = ctx.begin_render_pass("Main Pass");

        // Render skybox
        self.skybox.render(
            &mut render_pass,
            &self.skybox_bind_group,
            &self.skybox_texture_bind_group,
        );

        // Render sphere
        render_pass.set_pipeline(&self.model_pipeline);
        render_pass.set_bind_group(0, &self.model_bind_group, &[]);
        self.sphere.render(&mut render_pass);

        Ok(())
    }

    fn handle_event(&mut self, _ctx: &mut Context, event: &WindowEvent) -> bool {
        self.camera_controller.handle_event(event)
    }

    fn config() -> AppConfig {
        AppConfig {
            title: "RustForge Skybox Demo - New Framework".to_string(),
            width: 1024,
            height: 768,
            ..Default::default()
        }
    }
}

fn main() -> Result<()> {
    run::<SkyboxDemo>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uniforms_sizes() {
        assert_eq!(std::mem::size_of::<SkyboxUniforms>(), 64);
        assert_eq!(std::mem::size_of::<ModelUniforms>(), 224);
    }
}
