use std::sync::LazyLock;
use bytemuck::{Pod, Zeroable};
use crate::engine::renderer::camera::Camera2D;
use crate::engine::renderer::material::ShaderDefinition;
use crate::engine::renderer::Renderable;

pub static SPRITE_SHADER: LazyLock<ShaderDefinition> = LazyLock::new(|| ShaderDefinition {
    name: String::from("Sprite Shader"),
    source: String::from(include_str!("sprite_shader.wgsl"))
});

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct SpriteUniforms {
    pub local_to_world_model_matrix: [[f32; 4]; 4],
    pub world_to_view_matrix: [[f32; 4]; 4],
    pub view_to_clip_matrix: [[f32; 4]; 4]
}

impl SpriteUniforms {
    pub fn new(renderable: &Renderable, camera: &Camera2D) -> Self {
        Self {
            local_to_world_model_matrix: renderable.transform.local_to_world_model_matrix().to_cols_array_2d(),
            world_to_view_matrix: camera.world_to_view_matrix().to_cols_array_2d(),
            view_to_clip_matrix: camera.view_to_clip_matrix().to_cols_array_2d()
        }
    }
}