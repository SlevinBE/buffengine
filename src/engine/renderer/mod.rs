pub mod shaders;
pub mod wgpu;
pub mod material;
pub mod mesh;
pub mod transform;
pub mod camera;

use std::cell::Ref;
use material::Material;
use mesh::Mesh;
use transform::Transform2D;
use crate::engine::gameobjects::GameObject;
use crate::engine::renderer::camera::Camera2D;

pub trait Renderer {
    fn render(&mut self, renderables: &dyn Scene);
}

pub trait Scene {
    fn get_renderables(&self) -> Vec<&Renderable>;

    fn get_camera(&self) -> Ref<Camera2D>;
}

pub struct Renderable {
    pub name: String,
    pub mesh: Mesh,
    pub material: Material,
    pub transform: Transform2D // in world space
}