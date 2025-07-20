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

@vertex
fn vertex_shader(vertex: VertexData) -> VertexOutput {
    var output: VertexOutput;
    output.clip_position = vec4f(vertex.position, 1.0);
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