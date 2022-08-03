// Vertex shader
struct VertexInput {
    @location(0) index: f32,
    @location(1) position: vec2<f32>,
    @location(2) tex_coords: vec2<f32>,
    @location(3) color: vec4<f32>
};

struct VertexOutput 
{
    @builtin(position) clip_position: vec4<f32>,
    @location(0) index: f32,
    @location(1) tex_coords: vec2<f32>,
    @location(2) color: vec4<f32>
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput 
{
    var out: VertexOutput;
    out.index = model.index;
    out.tex_coords = model.tex_coords;
    out.clip_position = vec4<f32>(model.position, 0.0, 1.0);
    out.color = model.color;
    return out;
}

// Fragment shader
@group(0) @binding(0)
var texture_array: binding_array<texture_2d<f32>>;
@group(0) @binding(1)
var sampler_array: binding_array<sampler>;

@fragment
fn fs_main(in: VertexOutput) ->  @location(0) vec4<f32> 
{
    return in.color * textureSample(texture_array[in.index], sampler_array[in.index], in.tex_coords);
}