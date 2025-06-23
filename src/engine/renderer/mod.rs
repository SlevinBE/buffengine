mod triangle;
pub mod wgpu_renderer;

use std::ops::Range;
use wgpu::RenderPipeline;

pub trait Renderer {
    fn draw_triangle(&self);
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