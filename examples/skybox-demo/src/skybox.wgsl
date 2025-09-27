// Skybox shader for cubemap rendering

struct VertexInput {
    @location(0) position: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec3<f32>,
}

struct Uniforms {
    view_proj: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@group(1) @binding(0)
var skybox_texture: texture_cube<f32>;
@group(1) @binding(1)
var skybox_sampler: sampler;

@vertex
fn vs_main(vertex: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    // Use the vertex position as texture coordinates for cubemap
    out.tex_coords = vertex.position;

    // Transform position by view-projection matrix
    // Set w=0 to remove translation, keeping only rotation
    let pos = uniforms.view_proj * vec4<f32>(vertex.position, 0.0);

    // Ensure skybox is always at far plane
    out.clip_position = pos.xyww;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Sample the cubemap texture
    let color = textureSample(skybox_texture, skybox_sampler, in.tex_coords);
    return color;
}