use crate::engine::gameobjects::GameObject;
use crate::engine::renderer::{shaders, Material, Mesh, Renderable, ShaderDefinition, Texture, Vertex};
use crate::sample_game::resource_loader::load_texture_from_file;

pub struct SampleGameObject {
    renderable: Renderable
}

impl SampleGameObject {
    pub fn new() -> Self {
        let material = Material {
            shader: &shaders::SPRITE_SHADER,
            texture: Some(load_texture_from_file("src/sample_game/resources/warrior_idle.png", String::from("Warrior")).unwrap())
        };
        
        let mesh = Mesh {
            name: String::from("Sprite Rectangle Mesh"),
            vertices: vec!(
                //--- triangle 1
                // lower left
                Vertex {
                    position: [-0.5, -0.5, 0.0],
                    color: [1.0, 0.0, 0.0, 1.0],
                    tex_coords: [0.0, 0.0] 
                },
                // upper left
                Vertex {
                    position: [-0.5, 0.5, 0.0],
                    color: [0.0, 1.0, 0.0, 1.0],
                    tex_coords: [0.0, 1.0]
                },
                // upper right
                Vertex {
                    position: [0.5, 0.5, 0.0],
                    color: [0.0, 0.0, 1.0, 1.0],
                    tex_coords: [1.0, 1.0]
                },
                //--- triangle 2
                // lower left
                Vertex {
                    position: [-0.5, -0.5, 0.0],
                    color: [1.0, 0.0, 0.0, 1.0],
                    tex_coords: [0.0, 0.0]
                },
                // upper right
                Vertex {
                    position: [0.5, 0.5, 0.0],
                    color: [0.0, 0.0, 1.0, 1.0],
                    tex_coords: [1.0, 1.0]
                },
                // lower right
                Vertex {
                    position: [0.5, -0.5, 0.0],
                    color: [0.0, 0.0, 1.0, 1.0],
                    tex_coords: [1.0, 0.0]
                }
            )
        };
        
        let renderable = Renderable {
            name: String::from("Sprite"),
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