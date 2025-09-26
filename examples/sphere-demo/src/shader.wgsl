//! WGSL shader for sphere demo with lighting

struct Uniforms {
    model: mat4x4<f32>,
    view: mat4x4<f32>,
    projection: mat4x4<f32>,
    light_position: vec3<f32>,
    light_color: vec3<f32>,
    camera_position: vec3<f32>,
    _padding: f32,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
}

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    // Transform position to world space
    let world_position = uniforms.model * vec4<f32>(model.position, 1.0);
    out.world_position = world_position.xyz;

    // Transform normal to world space (assuming uniform scaling)
    out.world_normal = normalize((uniforms.model * vec4<f32>(model.normal, 0.0)).xyz);

    // Transform to clip space
    out.clip_position = uniforms.projection * uniforms.view * world_position;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Material properties
    let albedo = vec3<f32>(0.7, 0.3, 0.3); // Reddish material
    let ambient = 0.1;
    let specular_strength = 0.5;
    let shininess = 32.0;

    // Normalize inputs
    let normal = normalize(in.world_normal);
    let view_dir = normalize(uniforms.camera_position - in.world_position);
    let light_dir = normalize(uniforms.light_position - in.world_position);

    // Ambient lighting
    let ambient_lighting = ambient * albedo;

    // Diffuse lighting
    let diff = max(dot(normal, light_dir), 0.0);
    let diffuse_lighting = diff * albedo * uniforms.light_color;

    // Specular lighting
    let reflect_dir = reflect(-light_dir, normal);
    let spec = pow(max(dot(view_dir, reflect_dir), 0.0), shininess);
    let specular_lighting = specular_strength * spec * uniforms.light_color;

    // Combine lighting
    let final_color = ambient_lighting + diffuse_lighting + specular_lighting;

    return vec4<f32>(final_color, 1.0);
}
