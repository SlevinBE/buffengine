use crate::engine::core::layer::Layer;
use crate::engine::events::event::Event;

pub struct DebugOverlay {
    pub name: String
}

impl Layer for DebugOverlay {
    fn on_attach(&self) {
        println!("DebugOverlay attached");
    }

    fn on_detach(&self) {
        println!("DebugOverlay detached");
    }

    fn on_update(&self) {
        println!("DebugOverlay update");
    }

    fn on_event(&self, event: &Box<dyn Event>) -> bool {
        println!("DebugOverlay event: {:?}", event.get_event_type());
        false
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }
}