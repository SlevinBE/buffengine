use std::sync::LazyLock;
use crate::engine::renderer::ShaderDefinition;

pub static SPRITE_SHADER: LazyLock<ShaderDefinition> = LazyLock::new(|| ShaderDefinition {
    name: String::from("Sprite Shader"),
    source: String::from(include_str!("sprite_shader.wgsl"))
});