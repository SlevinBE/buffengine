struct VertexData {
    @location(0) position: vec3f
};

@vertex
fn vertex_shader(vertex: VertexData) -> @builtin(position) vec4f {
    return vec4f(vertex.position, 1.0);
}

@fragment
fn fragment_shader() -> @location(0) vec4f {
    return vec4f(1.0, 0.0, 0.0, 1.0);
}