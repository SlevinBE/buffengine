struct VertexData {
    @location(0) position: vec3f,
    @location(1) color: vec4f,
    @location(2) tex_coords: vec2f,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4f,
    @location(0) color: vec4f,
    @location(1) tex_coords: vec2f,
}

struct Uniforms {
    local_to_world_model_matrix: mat4x4<f32>,
    world_to_view_matrix: mat4x4<f32>,
    view_to_clip_matrix: mat4x4<f32>
}

@group(1) @binding(0)
var<uniform> uniforms: Uniforms;

@vertex
fn vertex_shader(vertex: VertexData) -> VertexOutput {
    var world_space = uniforms.local_to_world_model_matrix * vec4f(vertex.position, 1.0);
    var view_space = uniforms.world_to_view_matrix * world_space;

    var output: VertexOutput;
    output.clip_position = uniforms.view_to_clip_matrix * view_space;
    output.color = vertex.color;
    output.tex_coords = vertex.tex_coords;
    return output;
}

@group(0) @binding(0)
var texture: texture_2d<f32>;
@group(0) @binding(1)
var texture_sampler: sampler;

@fragment
fn fragment_shader(vertex: VertexOutput) -> @location(0) vec4f {
    let texture_color = textureSample(texture, texture_sampler, vertex.tex_coords);
    return texture_color * vertex.color;
}