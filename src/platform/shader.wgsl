// Vertex shader

struct CameraUniform {
    orthographic_projection: mat4x4<f32>;
};

[[group(1), binding(0)]]
var<uniform> camera: CameraUniform;

struct VertexInput {
    [[location(0)]] position: vec2<f32>;
    [[location(1)]] tex_coords: vec2<f32>;
    [[location(2)]] color: vec4<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] tex_coords: vec2<f32>;
    [[location(1)]] color: vec4<f32>;
};

[[stage(vertex)]]
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = camera.orthographic_projection * vec4<f32>(model.position, 0.0, 1.0);
    out.color = model.color;
    return out;
}

// Fragment shader

[[group(0), binding(0)]]
var t_diffuse: texture_2d<f32>;
[[group(0), binding(1)]]
var s_diffuse: sampler;

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return in.color * textureSample(t_diffuse, s_diffuse, in.tex_coords);
}