use crate::engine::events::Event;

pub struct WindowProps {
    pub title: String,
    pub width: u32,
    pub height: u32
}

impl Default for WindowProps {
    fn default() -> Self {
        Self {
            title: "BuffEngine".to_string(),
            width: 1280,
            height: 720
        }
    }
}