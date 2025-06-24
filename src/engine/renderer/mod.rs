pub mod wgpu_renderer;

use std::ops::Range;
use wgpu::RenderPipeline;

pub trait Renderer {
    fn render(&self, renderables: Vec<&Renderable>);
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
    pub position: [f32; 3]
}

pub struct Material {
    pub shader: ShaderDefinition
    
    // TODO: add texture support
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

pub trait RenderedObject {
    fn create_render_pipeline(&self, infra: &WgpuInfraPipeline) -> RenderPipeline;

    fn vertices(&self) -> Range<u32>;
}