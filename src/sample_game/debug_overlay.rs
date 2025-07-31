use log::debug;
use crate::engine::core::layer::Layer;
use crate::engine::events::Event;

pub struct DebugOverlay {
    pub name: String
}

impl Layer for DebugOverlay {
    fn update(&self) {
        debug!("DebugOverlay update");
    }

    fn handle_event(&self, event: &Event) -> bool {
        debug!("DebugOverlay event: {:?}", event);
        false
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }
}