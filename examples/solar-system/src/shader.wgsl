//! WGSL shader for solar system demo with lighting

struct Uniforms {
    view: mat4x4<f32>,
    projection: mat4x4<f32>,
    light_position: vec3<f32>,
    light_color: vec3<f32>,
    camera_position: vec3<f32>,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
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
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    // Reconstruct model matrix from instance data
    let model_matrix = mat4x4<f32>(
        input.model_matrix_0,
        input.model_matrix_1,
        input.model_matrix_2,
        input.model_matrix_3,
    );

    // Transform position to world space
    let world_position = model_matrix * vec4<f32>(input.position, 1.0);
    out.world_position = world_position.xyz;

    // Transform normal to world space
    out.world_normal = normalize((model_matrix * vec4<f32>(input.normal, 0.0)).xyz);

    // Pass through instance data
    out.color = input.color;
    out.emissive = input.emissive;

    // Transform to clip space
    out.clip_position = uniforms.projection * uniforms.view * world_position;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    if in.emissive > 0.5 {
        // Emissive object (sun) - no lighting needed
        return vec4<f32>(in.color * 2.0, 1.0);
    } else {
        // Planet with lighting
        let albedo = in.color;
        let ambient = 0.15;
        let specular_strength = 0.3;
        let shininess = 32.0;

        // Normalize inputs
        let normal = normalize(in.world_normal);
        let view_dir = normalize(uniforms.camera_position - in.world_position);
        let light_dir = normalize(uniforms.light_position - in.world_position);

        // Distance-based attenuation
        let light_distance = length(uniforms.light_position - in.world_position);
        let attenuation = 1.0 / (1.0 + 0.005 * light_distance + 0.0001 * light_distance * light_distance);

        // Ambient lighting
        let ambient_lighting = ambient * albedo;

        // Diffuse lighting
        let diff = max(dot(normal, light_dir), 0.0);
        let diffuse_lighting = diff * albedo * uniforms.light_color * attenuation;

        // Specular lighting
        let reflect_dir = reflect(-light_dir, normal);
        let spec = pow(max(dot(view_dir, reflect_dir), 0.0), shininess);
        let specular_lighting = specular_strength * spec * uniforms.light_color * attenuation;

        // Combine lighting
        let final_color = ambient_lighting + diffuse_lighting + specular_lighting;

        return vec4<f32>(final_color, 1.0);
    }
}
