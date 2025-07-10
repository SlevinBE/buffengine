use crate::engine::renderer::{Mesh, Renderable, RenderedObject, Renderer, ShaderDefinition, Vertex, WgpuInfraPipeline};
use std::iter::once;
use std::mem::offset_of;
use bytemuck::cast_slice;
use wgpu::{Buffer, BufferAddress, BufferUsages, ColorTargetState, FragmentState, InstanceDescriptor, RenderPass, RenderPipeline, RenderPipelineDescriptor, ShaderModule, ShaderModuleDescriptor, ShaderSource, TextureFormat, VertexAttribute, VertexBufferLayout, VertexFormat, VertexState, VertexStepMode};
use wgpu::StoreOp::Store;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
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
        ).unwrap();
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

    fn render_object(&self, renderable: &Renderable, render_pass: &mut RenderPass) {
        let shader_module = self.create_shader(&renderable.material.shader);
        let pipeline = self.create_pipeline(
            format!("{:?}-pipeline", renderable.name).as_str(), 
            shader_module
        );
        let vertex_buffer = self.create_vertex_buffer(&renderable.mesh);
        render_pass.set_pipeline(&pipeline);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.draw(0..renderable.mesh.vertices.len() as u32, 0..1);
    }
    
    fn create_shader(&self, shader_definition: &ShaderDefinition) -> ShaderModule {
        self.infra.device.create_shader_module(ShaderModuleDescriptor {
            label: Some(shader_definition.name.as_str()),
            source: ShaderSource::Wgsl(shader_definition.source.clone().into()),
        })
    }
    
    fn create_pipeline(&self, pipeline_name: &str, shader_module: ShaderModule) -> RenderPipeline {
        let preferred_format: TextureFormat = self.infra.surface.get_capabilities(&self.infra.adapter).formats[0];
        self.infra.device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some(pipeline_name),
            layout: None,
            vertex: VertexState {
                module: &shader_module,
                entry_point: None,
                compilation_options: Default::default(),
                buffers: &[VertexBufferLayout {
                    array_stride: size_of::<Vertex>() as BufferAddress,
                    step_mode: VertexStepMode::Vertex,
                    attributes: &[
                        VertexAttribute {
                            shader_location: 0, // position
                            format: VertexFormat::Float32x3,
                            offset: 0,
                        },
                        VertexAttribute {
                            shader_location: 1, // position
                            format: VertexFormat::Float32x4,
                            offset: offset_of!(Vertex, color) as BufferAddress,
                        }
                    ]
                }
                ],
            },
            fragment: Some(FragmentState {
                module: &shader_module,
                entry_point: None,
                compilation_options: Default::default(),
                targets: &[Some(ColorTargetState {
                    format: preferred_format,
                    blend: None,
                    write_mask: Default::default(),
                })]
            }),
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            multiview: None,
            cache: None,
        })
    }

    fn create_vertex_buffer(&self, mesh: &Mesh) -> Buffer {
        self.infra.device.create_buffer_init(&BufferInitDescriptor {
            label: Some(format!("{:?}-vertex-buffer", mesh.name).as_str()),
            contents: cast_slice(mesh.vertices.as_slice()),
            usage: BufferUsages::VERTEX
        })
    }
}

impl <'window> Renderer for WgpuRenderer<'window> {
    
    fn render(&self, renderables: Vec<&Renderable>) {
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
            
            for renderable in renderables {
                self.render_object(renderable, &mut render_pass);    
            }
        }

        self.infra.queue.submit(once(encoder.finish()));
        frame.present();
    }

}