use crate::engine::core::layer::Layer;
use crate::engine::events::event::Event;

pub struct SampleLayer {
    pub name: String
}

impl Layer for SampleLayer {

    fn on_attach(&self) {
        println!("SampleLayer attached");
    }

    fn on_detach(&self) {
        println!("SampleLayer detached");
    }

    fn on_update(&self) {
        println!("SampleLayer update");
    }

    fn on_event(&self, event: &Box<dyn Event>) -> bool{
        println!("SampleLayer event: {:?}", event.get_event_type());
        false
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }
}