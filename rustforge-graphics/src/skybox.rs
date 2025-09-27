//! High-level skybox abstraction

use crate::vertex::Position;
use image::GenericImageView;
use rustforge_core::prelude::*;
use wgpu::util::DeviceExt;

/// A skybox with cubemap textures
pub struct Skybox {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    #[allow(dead_code)]
    texture: wgpu::Texture,
    texture_view: wgpu::TextureView,
    sampler: wgpu::Sampler,
    pipeline: wgpu::RenderPipeline,
    bind_group_layout: wgpu::BindGroupLayout,
    texture_bind_group_layout: wgpu::BindGroupLayout,
}

impl Skybox {
    /// Create a skybox from cubemap images
    pub fn from_cubemap(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        cubemap_dir: &str,
    ) -> Result<Self> {
        // Create vertices for skybox cube
        let vertices = Self::create_vertices();
        let indices = Self::create_indices();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Skybox Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Skybox Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        // Load cubemap texture
        let texture = Self::load_cubemap_texture(device, queue, cubemap_dir)?;
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::Cube),
            ..Default::default()
        });

        // Create sampler
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        // Create shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Skybox Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/skybox.wgsl").into()),
        });

        // Create bind group layouts
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("skybox_bind_group_layout"),
        });

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::Cube,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("skybox_texture_bind_group_layout"),
            });

        // Create pipeline
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Skybox Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout, &texture_bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Skybox Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Position::desc()],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: false,
                depth_compare: wgpu::CompareFunction::LessEqual,
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

        Ok(Self {
            vertex_buffer,
            index_buffer,
            num_indices: indices.len() as u32,
            texture,
            texture_view,
            sampler,
            pipeline,
            bind_group_layout,
            texture_bind_group_layout,
        })
    }

    /// Render the skybox
    pub fn render<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        uniforms_bind_group: &'a wgpu::BindGroup,
        texture_bind_group: &'a wgpu::BindGroup,
    ) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, uniforms_bind_group, &[]);
        render_pass.set_bind_group(1, texture_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }

    /// Get bind group layout for uniforms
    pub fn uniform_bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    /// Create bind group for textures
    pub fn create_texture_bind_group(&self, device: &wgpu::Device) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&self.texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
            ],
            label: Some("skybox_texture_bind_group"),
        })
    }

    fn create_vertices() -> Vec<Position> {
        vec![
            // Front face (-Z)
            Position::new([-1.0, -1.0, -1.0]),
            Position::new([1.0, -1.0, -1.0]),
            Position::new([1.0, 1.0, -1.0]),
            Position::new([-1.0, 1.0, -1.0]),
            // Back face (+Z)
            Position::new([-1.0, -1.0, 1.0]),
            Position::new([-1.0, 1.0, 1.0]),
            Position::new([1.0, 1.0, 1.0]),
            Position::new([1.0, -1.0, 1.0]),
        ]
    }

    fn create_indices() -> Vec<u16> {
        vec![
            0, 1, 2, 2, 3, 0, // front
            4, 5, 6, 6, 7, 4, // back
            0, 3, 5, 5, 4, 0, // left
            1, 7, 6, 6, 2, 1, // right
            3, 2, 6, 6, 5, 3, // top
            0, 4, 7, 7, 1, 0, // bottom
        ]
    }

    fn load_cubemap_texture(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        cubemap_dir: &str,
    ) -> Result<wgpu::Texture> {
        let face_paths = [
            format!("{}/px.png", cubemap_dir), // Right
            format!("{}/nx.png", cubemap_dir), // Left
            format!("{}/py.png", cubemap_dir), // Top
            format!("{}/ny.png", cubemap_dir), // Bottom
            format!("{}/pz.png", cubemap_dir), // Front
            format!("{}/nz.png", cubemap_dir), // Back
        ];

        // Load first image to get dimensions
        let first_img = image::open(&face_paths[0]).map_err(|e| {
            Error::Graphics(format!(
                "Failed to load cubemap face {}: {}",
                face_paths[0], e
            ))
        })?;
        let dimensions = first_img.dimensions();

        // Create cubemap texture
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Skybox Cubemap"),
            size: wgpu::Extent3d {
                width: dimensions.0,
                height: dimensions.1,
                depth_or_array_layers: 6,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        // Load and write each face
        for (i, path) in face_paths.iter().enumerate() {
            let img = image::open(path)
                .map_err(|e| Error::Graphics(format!("Failed to load {}: {}", path, e)))?;
            let rgba = img.to_rgba8();

            queue.write_texture(
                wgpu::ImageCopyTexture {
                    texture: &texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d {
                        x: 0,
                        y: 0,
                        z: i as u32,
                    },
                    aspect: wgpu::TextureAspect::All,
                },
                &rgba,
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(4 * dimensions.0),
                    rows_per_image: Some(dimensions.1),
                },
                wgpu::Extent3d {
                    width: dimensions.0,
                    height: dimensions.1,
                    depth_or_array_layers: 1,
                },
            );
        }

        Ok(texture)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vertices() {
        let vertices = Skybox::create_vertices();
        assert_eq!(vertices.len(), 8); // Cube has 8 vertices
    }

    #[test]
    fn test_create_indices() {
        let indices = Skybox::create_indices();
        assert_eq!(indices.len(), 36); // 6 faces * 2 triangles * 3 vertices
    }
}
