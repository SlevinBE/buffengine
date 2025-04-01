use std::ops::Range;
use wgpu::{ColorTargetState, FragmentState, RenderPass, RenderPipeline, RenderPipelineDescriptor, ShaderModule, ShaderModuleDescriptor, ShaderSource, TextureFormat, VertexState};
use crate::engine::renderer::{RenderedObject, Renderer, WgpuInfraPipeline};

pub struct TriangleDefinition<'a> {
    label: String,
    shader: ShaderSource<'a>
}

impl <'a> TriangleDefinition<'a> {

    pub fn new() -> TriangleDefinition<'a> {
        TriangleDefinition {
            label: String::from("Triangle"),
            shader: ShaderSource::Wgsl(include_str!("shaders/triangle.wgsl").into())
        }
    }

}

impl <'a> RenderedObject for TriangleDefinition<'a> {
    fn create_render_pipeline(&self, infra: &WgpuInfraPipeline) -> RenderPipeline {
        let module: ShaderModule = infra.device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Triangle Shader"),
            source: ShaderSource::Wgsl(include_str!("shaders/triangle.wgsl").into()),
        });
        let preferred_format: TextureFormat = infra.surface.get_capabilities(&infra.adapter).formats[0];
        let pipeline = infra.device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("triangle pipeline"),
            layout: None,
            vertex: VertexState {
                module: &module,
                entry_point: None,
                compilation_options: Default::default(),
                buffers: &[],
            },
            fragment: Some(FragmentState {
                module: &module,
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
        });

        pipeline
    }

    fn vertices(&self) -> Range<u32> {
        0..3
    }
}