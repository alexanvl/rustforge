// Model shader - simple lit sphere

struct Uniforms {
    model: mat4x4<f32>,
    view: mat4x4<f32>,
    projection: mat4x4<f32>,
    light_position: vec3<f32>,
    _padding1: f32,
    camera_position: vec3<f32>,
    _padding2: f32,
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
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    let world_position = uniforms.model * vec4<f32>(input.position, 1.0);
    output.world_position = world_position.xyz;
    output.world_normal = normalize((uniforms.model * vec4<f32>(input.normal, 0.0)).xyz);

    let view_position = uniforms.view * world_position;
    output.clip_position = uniforms.projection * view_position;

    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let ambient = 0.1;
    let sphere_color = vec3<f32>(0.7, 0.7, 0.9);

    // Simple diffuse lighting
    let light_dir = normalize(uniforms.light_position - input.world_position);
    let normal = normalize(input.world_normal);
    let diffuse = max(dot(normal, light_dir), 0.0);

    // View-dependent specular
    let view_dir = normalize(uniforms.camera_position - input.world_position);
    let reflect_dir = reflect(-light_dir, normal);
    let specular = pow(max(dot(view_dir, reflect_dir), 0.0), 32.0);

    let final_color = sphere_color * (ambient + diffuse) + vec3<f32>(specular);

    return vec4<f32>(final_color, 1.0);
}
