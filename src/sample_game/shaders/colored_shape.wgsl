struct VertexData {
    @location(0) position: vec3f,
    @location(1) color: vec4f
};

struct VertexOutput {
    @builtin(position) clip_position: vec4f,
    @location(0) color: vec4f
}

@vertex
fn vertex_shader(vertex: VertexData) -> VertexOutput {
    var output: VertexOutput;
    output.clip_position = vec4f(vertex.position, 1.0);
    output.color = vertex.color;
    return output;
}

@fragment
fn fragment_shader(vertex: VertexOutput) -> @location(0) vec4f {
    return vec4f(vertex.color);
}