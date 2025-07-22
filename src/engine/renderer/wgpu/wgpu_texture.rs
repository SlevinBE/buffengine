use crate::engine::renderer::material::Texture;

pub struct WgpuTexture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub bind_group: wgpu::BindGroup
}

impl WgpuTexture {
    
    const RGBA_BYTES: u32 = (size_of::<u8>() as u32 * 4);
    
    pub fn from_abstract_texture(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        abstract_texture: &Texture,
        bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(&abstract_texture.name),
            size: wgpu::Extent3d {
                width: abstract_texture.width,
                height: abstract_texture.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            view_formats: &[wgpu::TextureFormat::Rgba8UnormSrgb],
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        });
        
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &abstract_texture.data,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(Self::RGBA_BYTES * abstract_texture.width),
                rows_per_image: Some(abstract_texture.height),
            },
            wgpu::Extent3d {
                width: abstract_texture.width,
                height: abstract_texture.height,
                depth_or_array_layers: 1,
            },
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: Some(&format!("{}_bind_group", abstract_texture.name)),
        });

        Self {
            texture,
            view,
            sampler,
            bind_group,
        }
    }
}
