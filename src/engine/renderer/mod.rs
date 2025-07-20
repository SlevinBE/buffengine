pub mod wgpu_renderer;
pub mod shaders;
mod wgpu_texture;

use std::ops::Range;
use wgpu::RenderPipeline;

pub trait Renderer {
    fn render(&mut self, renderables: Vec<&Renderable>);
}

pub struct Renderable {
    pub name: String,
    pub mesh: Mesh,
    pub material: Material
}

pub struct Mesh {
    pub name: String,
    pub vertices: Vec<Vertex>
}

#[repr(C)]  // Guarantees consistent memory layout across platforms
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
    pub tex_coords: [f32; 2] // UV coordinates
}

pub struct Material {
    pub shader: &'static ShaderDefinition,
    pub texture: Option<Texture>
}

pub struct Texture {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8> // RGBA pixel data
}

pub struct ShaderDefinition {
    pub name: String,
    pub source: String
}

pub struct WgpuInfraPipeline<'window> {
    surface: wgpu::Surface<'window>,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}