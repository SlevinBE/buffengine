#[derive(Debug, Clone)]
pub struct Mesh {
    pub name: String,
    pub vertices: Vec<Vertex>
}

#[repr(C)]  // Guarantees consistent memory layout across platforms
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3], // local space coordinates (-0.5 - 0.5)
    pub color: [f32; 4],
    pub tex_coords: [f32; 2] // UV coordinates (0 - 1)
}