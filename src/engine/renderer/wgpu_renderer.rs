use crate::engine::renderer::triangle::TriangleDefinition;
use crate::engine::renderer::{RenderedObject, Renderer, WgpuInfraPipeline};
use std::iter::once;
use wgpu::InstanceDescriptor;
use wgpu::StoreOp::Store;
use winit::window::Window;

pub struct WgpuRenderer<'window> {
    infra: WgpuInfraPipeline<'window>,
}

impl <'window> WgpuRenderer<'window> {
    pub fn new(window: Window) -> WgpuRenderer<'window> {
        let instance_descriptor = InstanceDescriptor::default();
        let instance = wgpu::Instance::new(&instance_descriptor);
        let size = window.inner_size();
        let surface = unsafe { instance.create_surface(window).unwrap() };
        let adapter = pollster::block_on(
            instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
        )
            .unwrap();
        let (device, queue) = pollster::block_on(
            adapter.request_device(&wgpu::DeviceDescriptor::default(), None)
        )
            .unwrap();
        if let Some(config) = surface.get_default_config(&adapter, size.width, size.height) {
            surface.configure(&device, &config);
        }

        WgpuRenderer {
            infra: WgpuInfraPipeline {
                surface,
                adapter,
                device,
                queue,
            }
        }
    }
}

impl <'window> Renderer for WgpuRenderer<'window> {

    fn draw(&self) {
        let triangle_definition = TriangleDefinition::new();
        let pipeline = triangle_definition.create_render_pipeline(&self.infra);

        let frame = self.infra.surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let render_pass_desc = wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    }),
                    store: Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None
        };
        let mut encoder =
            self.infra.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&render_pass_desc);
            render_pass.set_pipeline(&pipeline);
            render_pass.draw(triangle_definition.vertices(), 0..1);
        }

        self.infra.queue.submit(once(encoder.finish()));
        frame.present();
    }
}