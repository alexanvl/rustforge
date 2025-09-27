//! 2D text rendering system for HUD and UI elements

use rustforge_core::prelude::*;
use std::sync::Arc;

// Font atlas constants
const CHARS_PER_ROW: u32 = 16;
const CHAR_WIDTH: u32 = 24;
const CHAR_HEIGHT: u32 = 32;
const ATLAS_ROWS: u32 = 8;

/// 2D text renderer using fontdue for proper font rendering
pub struct TextRenderer {
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    font_texture: wgpu::Texture,
    font_view: wgpu::TextureView,
    sampler: wgpu::Sampler,
    bind_group: wgpu::BindGroup,

    font: fontdue::Font,
    vertices: Vec<TextVertex>,
    indices: Vec<u32>,
    max_chars: usize,
    surface_width: u32,
    surface_height: u32,
}

/// Vertex for text rendering
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TextVertex {
    pub position: [f32; 2],
    pub uv: [f32; 2],
    pub color: [f32; 4],
}

impl TextRenderer {
    /// Create a new text renderer using fontdue with embedded font
    pub fn new(device: &Arc<wgpu::Device>, queue: &Arc<wgpu::Queue>, surface_format: wgpu::TextureFormat) -> Result<Self> {
        // Load default font
        let font_data = include_bytes!("../assets/Geneva.ttf");
        Self::new_with_font(device, queue, surface_format, font_data)
    }

    /// Create a new text renderer using fontdue with provided font data
    pub fn new_with_font(device: &Arc<wgpu::Device>, queue: &Arc<wgpu::Queue>, surface_format: wgpu::TextureFormat, font_data: &[u8]) -> Result<Self> {
        let max_chars = 1024;
        let surface_width = 800;
        let surface_height = 600;

        // Load the provided font
        let font = fontdue::Font::from_bytes(font_data, fontdue::FontSettings::default())
            .map_err(|e| Error::Graphics(format!("Failed to load font: {:?}", e)))?;

        // Create font atlas texture
        let font_size = 24.0;
        let texture_width = CHARS_PER_ROW * CHAR_WIDTH;
        let texture_height = ATLAS_ROWS * CHAR_HEIGHT;

        let font_texture = device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: texture_width,
                height: texture_height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::R8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
            label: Some("Font Texture"),
        });

        // Generate font atlas
        let mut font_pixels = vec![0u8; (texture_width * texture_height) as usize];
        let ascii_chars = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";

                    for (i, ch) in ascii_chars.chars().enumerate() {
                        let i_u32 = i as u32;
                        if i_u32 >= CHARS_PER_ROW * ATLAS_ROWS { break; }

                        let char_x = (i_u32 % CHARS_PER_ROW) * CHAR_WIDTH;
                        let char_y = (i_u32 / CHARS_PER_ROW) * CHAR_HEIGHT;

            let (metrics, bitmap) = font.rasterize(ch, font_size);
            let bitmap_width = metrics.width as usize;
            let bitmap_height = metrics.height as usize;

                        // Copy bitmap to texture atlas
                        for y in 0..bitmap_height.min(CHAR_HEIGHT as usize) {
                            for x in 0..bitmap_width.min(CHAR_WIDTH as usize) {
                                let src_idx = y * bitmap_width + x;
                                let dst_x = char_x + x as u32;
                                let ymin_offset = if metrics.ymin < 0 { 0 } else { metrics.ymin as u32 };
                                let dst_y = char_y + y as u32 + ymin_offset;
                                if dst_x < texture_width && dst_y < texture_height && src_idx < bitmap.len() {
                                    let dst_idx = (dst_y as usize * texture_width as usize + dst_x as usize) as usize;
                                    font_pixels[dst_idx] = bitmap[src_idx];
                                }
                            }
                        }
        }

        // Upload font data to texture
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &font_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &font_pixels,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(texture_width),
                rows_per_image: Some(texture_height),
            },
            wgpu::Extent3d {
                width: texture_width,
                height: texture_height,
                depth_or_array_layers: 1,
            },
        );

        let font_view = font_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        // Create bind group
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
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
            label: Some("text_bind_group_layout"),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&font_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: Some("text_bind_group"),
        });

        // Create vertex and index buffers
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Text Vertex Buffer"),
            size: (std::mem::size_of::<TextVertex>() * max_chars * 4) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Text Index Buffer"),
            size: (std::mem::size_of::<u32>() * max_chars * 6) as u64,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create shader
        let shader_source = include_str!("shaders/text.wgsl");
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Text Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        // Create pipeline
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Text Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Text Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<TextVertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x2,
                        },
                        wgpu::VertexAttribute {
                            offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                            shader_location: 1,
                            format: wgpu::VertexFormat::Float32x2,
                        },
                        wgpu::VertexAttribute {
                            offset: (std::mem::size_of::<[f32; 2]>() + std::mem::size_of::<[f32; 2]>()) as wgpu::BufferAddress,
                            shader_location: 2,
                            format: wgpu::VertexFormat::Float32x4,
                        },
                    ],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                        alpha: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::One,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
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
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Ok(Self {
            device: device.clone(),
            queue: queue.clone(),
            pipeline,
            vertex_buffer,
            index_buffer,
            font_texture,
            font_view,
            sampler,
            bind_group,
            font,
            vertices: Vec::new(),
            indices: Vec::new(),
            max_chars,
            surface_width,
            surface_height,
        })
    }

    /// Add text to be rendered
    pub fn add_text(&mut self, text: &str, position: glam::Vec2, color: [f32; 4], scale: f32) {
        let mut cursor_x = position.x;
        let mut cursor_y = position.y;
        let font_size = 24.0 * scale;
        let line_height = font_size * 1.2;

        for ch in text.chars() {
            if ch == '\n' {
                cursor_x = position.x;
                cursor_y += line_height;
                continue;
            }

            if ch == ' ' {
                cursor_x += font_size * 0.6;
                continue;
            }

            // Rasterize character
            let (metrics, bitmap) = self.font.rasterize(ch, font_size);

            if !bitmap.is_empty() {
                let char_x = cursor_x + metrics.xmin as f32;
                let char_y = cursor_y + metrics.ymin as f32;

                // Find character index in our ASCII set
                let ascii_chars = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
                let char_index = ascii_chars.find(ch).unwrap_or(0) as u32;

                let atlas_x = (char_index % CHARS_PER_ROW) * CHAR_WIDTH;
                let atlas_y = (char_index / CHARS_PER_ROW) * CHAR_HEIGHT;

                let uv_min_x = atlas_x as f32 / (CHARS_PER_ROW * CHAR_WIDTH) as f32;
                let uv_min_y = atlas_y as f32 / (ATLAS_ROWS * CHAR_HEIGHT) as f32;
                let uv_max_x = (atlas_x + metrics.width as u32) as f32 / (CHARS_PER_ROW * CHAR_WIDTH) as f32;
                let uv_max_y = (atlas_y + metrics.height as u32) as f32 / (ATLAS_ROWS * CHAR_HEIGHT) as f32;

                // Add quad for this character
                let base_index = self.vertices.len() as u32;

                self.vertices.extend_from_slice(&[
                    TextVertex {
                        position: [char_x, char_y],
                        uv: [uv_min_x, uv_min_y],
                        color,
                    },
                    TextVertex {
                        position: [char_x + metrics.width as f32, char_y],
                        uv: [uv_max_x, uv_min_y],
                        color,
                    },
                    TextVertex {
                        position: [char_x + metrics.width as f32, char_y + metrics.height as f32],
                        uv: [uv_max_x, uv_max_y],
                        color,
                    },
                    TextVertex {
                        position: [char_x, char_y + metrics.height as f32],
                        uv: [uv_min_x, uv_max_y],
                        color,
                    },
                ]);

                self.indices.extend_from_slice(&[
                    base_index, base_index + 1, base_index + 2,
                    base_index, base_index + 2, base_index + 3,
                ]);
            }

            cursor_x += metrics.advance_width;
        }
    }

    /// Render all queued text
    pub fn render<'a>(&'a mut self, render_pass: &mut wgpu::RenderPass<'a>) -> Result<()> {
        if self.vertices.is_empty() {
            return Ok(());
        }

        // Update buffers
        self.queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&self.vertices));
        self.queue.write_buffer(&self.index_buffer, 0, bytemuck::cast_slice(&self.indices));

        // Render
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.draw_indexed(0..self.indices.len() as u32, 0, 0..1);

        // Clear for next frame
        self.vertices.clear();
        self.indices.clear();

        Ok(())
    }

    /// Resize the text renderer
    pub fn resize(&mut self, width: u32, height: u32) {
        self.surface_width = width;
        self.surface_height = height;
    }

    /// Clear all queued text for the next frame
    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
    }
}
