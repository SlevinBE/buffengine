use crate::engine::renderer::{shaders, Renderable};
use crate::engine::renderer::material::Material;
use crate::engine::renderer::mesh::{Mesh, Vertex};
use crate::engine::renderer::transform::Transform2D;
use crate::sample_game::resource_loader::load_texture_from_file;

#[derive(Debug, Clone)]
pub struct Sprite {
    renderable: Renderable
}

impl Sprite {
    /// # Parameters
    /// * `x` - position on the x-axis in world units
    /// * `y` - position on the y-axis in world units
    /// * `scale` - the scale in world units
    pub fn new(x: f32, y: f32, scale: f32) -> Self {
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
            material,
            transform: Transform2D {
                position: [x, y],
                scale: [1.0 * scale, 1.0 * scale]
            }
        };
        
        Self {
            renderable
        }
    }

    pub fn get_renderable(&self) -> &Renderable {
        &self.renderable
    }
    
    fn move_xy(&mut self, offset_x: f32, offset_y: f32) {
        self.renderable.transform = self.renderable.transform.position_to(self.renderable.transform.position[0] + offset_x, self.renderable.transform.position[1] + offset_y);
    }
}
impl Movable for Sprite {
    
    fn move_up(&mut self, offset: f32) {
        self.move_xy(0.0, offset)
    }

    fn move_down(&mut self, offset: f32) {
        self.move_xy(0.0, -offset)
    }

    fn move_left(&mut self, offset: f32) {
        self.move_xy(-offset, 0.0)
    }

    fn move_right(&mut self, offset: f32) {
        self.move_xy(offset, 0.0)
    }
    
}

pub trait Movable {
    fn move_up(&mut self, offset: f32);
    fn move_down(&mut self, offset: f32);
    fn move_left(&mut self, offset: f32);
    fn move_right(&mut self, offset: f32);
}