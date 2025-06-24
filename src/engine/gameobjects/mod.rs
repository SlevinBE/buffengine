use crate::engine::renderer::Renderable;

pub trait GameObject {
    fn get_renderable(&self) -> &Renderable;
}