use crate::engine::gameobjects::GameObject;
use crate::engine::renderer::{Material, Mesh, Renderable, ShaderDefinition, Vertex};

pub struct SampleGameObject {
    renderable: Renderable
}

impl SampleGameObject {
    pub fn new() -> Self {
        let shader_definition = ShaderDefinition {
            name: String::from("Triangle Shader"),
            source: String::from(include_str!("shaders/colored_shape.wgsl"))
        };
        
        let material = Material {
            shader: shader_definition
        };
        
        let mesh = Mesh {
            name: String::from("Triangle"),
            vertices: vec!(
                Vertex {
                    position: [0.0, 0.5, 0.0],
                },
                Vertex {
                    position: [-0.5, -0.5, 0.0],
                },
                Vertex {
                    position: [0.5, -0.5, 0.0],
                }
                
            )
        };
        
        let renderable = Renderable {
            name: String::from("Triangle"),
            mesh,
            material
        };
        
        Self {
            renderable
        }
    }
}
impl GameObject for SampleGameObject {
    
    fn get_renderable(&self) -> &Renderable {
        &self.renderable
    }
}