// Solar System shader with instanced rendering

struct Uniforms {
    view: mat4x4<f32>,
    projection: mat4x4<f32>,
    light_position: vec3<f32>,
    _padding1: f32,
    light_color: vec3<f32>,
    _padding2: f32,
    camera_position: vec3<f32>,
    _padding3: f32,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
}

struct InstanceInput {
    @location(2) model_matrix_0: vec4<f32>,
    @location(3) model_matrix_1: vec4<f32>,
    @location(4) model_matrix_2: vec4<f32>,
    @location(5) model_matrix_3: vec4<f32>,
    @location(6) color: vec3<f32>,
    @location(7) emissive: f32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) color: vec3<f32>,
    @location(3) emissive: f32,
}

@vertex
fn vs_main(vertex: VertexInput, instance: InstanceInput) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3
    );

    var output: VertexOutput;
    let world_position = model_matrix * vec4<f32>(vertex.position, 1.0);
    output.world_position = world_position.xyz;
    output.world_normal = normalize((model_matrix * vec4<f32>(vertex.normal, 0.0)).xyz);
    output.color = instance.color;
    output.emissive = instance.emissive;

    let view_position = uniforms.view * world_position;
    output.clip_position = uniforms.projection * view_position;

    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Emissive objects (sun) glow
    if (input.emissive > 0.5) {
        return vec4<f32>(input.color * 1.5, 1.0);
    }

    // Lighting for planets
    let ambient = 0.1;
    let light_dir = normalize(uniforms.light_position - input.world_position);
    let normal = normalize(input.world_normal);
    let diffuse = max(dot(normal, light_dir), 0.0);

    // Simple distance-based light falloff
    let distance = length(uniforms.light_position - input.world_position);
    let attenuation = 1.0 / (1.0 + 0.09 * distance + 0.032 * distance * distance);

    let lit_color = input.color * (ambient + diffuse * attenuation * uniforms.light_color);

    return vec4<f32>(lit_color, 1.0);
}
