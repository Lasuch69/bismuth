// Vertex shader

struct UniformBufferObject {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> ubo: UniformBufferObject;

struct VertexInput {
	@location(0) position: vec3<f32>,
	@location(1) color: vec3<f32>,
	@location(2) tex_coords: vec2<f32>
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
	@location(0) color: vec3<f32>,
	@location(1) tex_coords: vec2<f32>
};

@vertex
fn vertex(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.position = ubo.view_proj * vec4<f32>(in.position, 1.0);
    out.color = in.color;
    out.tex_coords = in.tex_coords;

    return out;
}

// Fragment shader

@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
