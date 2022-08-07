struct VertexInput 
{
    @location(0) position: vec2<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) color: vec4<f32>,
};

struct VertexOutput 
{
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput 
{
    var out: VertexOutput;
    
    out.uv = in.uv;
    out.color = in.color;
    out.clip_position = vec4<f32>(in.position, 0.0, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) ->  @location(0) vec4<f32> 
{
    var fade = 0.060;
    var thickness = 0.002;
    var uv = in.uv * 2.0 - 1.0;

    var distance = 1.0 - length(uv);
    var alpha = smoothstep(0.0, fade, distance);
    alpha *= smoothstep(thickness + fade, thickness, distance);


    return vec4<f32>(in.color.xyz, alpha);
}