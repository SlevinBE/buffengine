#[derive(Debug, Clone)]
pub struct Material {
    pub shader: &'static ShaderDefinition,
    pub texture: Option<Texture>
}

#[derive(Debug, Clone)]
pub struct Texture {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8> // RGBA pixel data
}

#[derive(Debug, Clone)]
pub struct ShaderDefinition {
    pub name: String,
    pub source: String
}