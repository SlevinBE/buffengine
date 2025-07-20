use std::collections::HashMap;
use crate::engine::renderer::{Mesh, Renderable, Renderer, ShaderDefinition, Texture, Vertex, WgpuInfraPipeline};
use std::iter::once;
use std::mem::offset_of;
use bytemuck::cast_slice;
use wgpu::{Buffer, BufferAddress, BufferUsages, ColorTargetState, FragmentState, InstanceDescriptor, RenderPass, RenderPipeline, RenderPipelineDescriptor, ShaderModule, ShaderModuleDescriptor, ShaderSource, TextureFormat, VertexAttribute, VertexBufferLayout, VertexFormat, VertexState, VertexStepMode};
use wgpu::StoreOp::Store;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use winit::window::Window;
use crate::engine::renderer::wgpu_texture::WgpuTexture;

pub struct WgpuRenderer<'window> {
    infra: WgpuInfraPipeline<'window>,
    texture_cache: HashMap<String, WgpuTexture>,
    texture_bind_group_layout: wgpu::BindGroupLayout,   
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
        ).unwrap();
        if let Some(config) = surface.get_default_config(&adapter, size.width, size.height) {
            surface.configure(&device, &config);
        }
        
        let texture_bind_group_layout = WgpuRenderer::create_texture_bind_group_layout(&device);

        WgpuRenderer {
            infra: WgpuInfraPipeline {
                surface,
                adapter,
                device,
                queue,
            },
            texture_cache: HashMap::new(),
            texture_bind_group_layout
        }
    }

    fn render_object(&mut self, renderable: &Renderable, render_pass: &mut RenderPass) {
        let shader_module = self.create_shader(&renderable.material.shader);
        let pipeline = self.create_pipeline(
            format!("{:?}-pipeline", renderable.name).as_str(), 
            shader_module
        );
        let vertex_buffer = self.create_vertex_buffer(&renderable.mesh);
        render_pass.set_pipeline(&pipeline);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));

        // Bind texture if available
        if let Some(ref abstract_texture) = renderable.material.texture {
            let wgpu_texture = self.get_or_create_texture(abstract_texture);
            render_pass.set_bind_group(0, &wgpu_texture.bind_group, &[]);
        }

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

        let pipeline_layout = self.infra.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&self.texture_bind_group_layout],
            push_constant_ranges: &[]
        });

        self.infra.device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some(pipeline_name),
            layout: Some(&pipeline_layout),
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
                            shader_location: 1, // color
                            format: VertexFormat::Float32x4,
                            offset: offset_of!(Vertex, color) as BufferAddress,
                        },
                        VertexAttribute {
                            shader_location: 2, // tex_coords
                            format: VertexFormat::Float32x2,
                            offset: offset_of!(Vertex, tex_coords) as BufferAddress,       
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
    
    fn create_texture_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
            })
    }

    fn get_or_create_texture(&mut self, abstract_texture: &Texture) -> &WgpuTexture {
        if !self.texture_cache.contains_key(&abstract_texture.name) {
            let wgpu_texture = WgpuTexture::from_abstract_texture(
                &self.infra.device,
                &self.infra.queue,
                abstract_texture,
                &self.texture_bind_group_layout,
            );
            self.texture_cache.insert(abstract_texture.name.clone(), wgpu_texture);
        }
        self.texture_cache.get(&abstract_texture.name).unwrap()
    }

}

impl <'window> Renderer for WgpuRenderer<'window> {
    
    fn render(&mut self, renderables: Vec<&Renderable>) {
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
                // TODO: this might be problematic, as we draw multiple times before submitting the command buffer.
                // so it might only draw the last renderable we added to the command buffer.
                // see https://webgpufundamentals.org/webgpu/lessons/webgpu-uniforms.html
                self.render_object(renderable, &mut render_pass);    
            }
        }

        self.infra.queue.submit(once(encoder.finish()));
        frame.present();
    }

}