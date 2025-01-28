use crate::engine::events::event::Event;

pub trait Layer {
    fn on_attach(&self);
    fn on_detach(&self);
    fn on_update(&self);
    fn on_event(&self, event: &Box<dyn Event>) -> bool;
    fn get_name(&self) -> &str;
}