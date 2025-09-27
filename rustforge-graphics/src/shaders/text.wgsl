// Text rendering shader

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
};

@group(0) @binding(0)
var font_texture: texture_2d<f32>;

@group(0) @binding(1)
var font_sampler: sampler;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    // Convert screen coordinates to clip space (-1 to 1)
    // Screen coordinates are in pixels, convert to NDC
    let screen_width = 800.0;
    let screen_height = 600.0;

    output.position = vec4<f32>(
        (input.position.x / screen_width) * 2.0 - 1.0,
        1.0 - (input.position.y / screen_height) * 2.0,  // Flip Y axis
        0.0,
        1.0
    );
    output.uv = input.uv;
    output.color = input.color;
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let alpha = textureSample(font_texture, font_sampler, input.uv).r;
    return vec4<f32>(input.color.rgb, input.color.a * alpha);
}