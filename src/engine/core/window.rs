use crate::engine::events::Event;
use std::sync::mpsc::Receiver;

pub trait Window {
    fn update(&mut self);
    
    fn get_width(&self) -> u32;
    
    fn get_height(&self) -> u32;
    
    fn set_vsync(&mut self, enabled: bool);
    
    fn is_vsync_enabled(&self) -> bool;
    
    fn events(&self) -> &Receiver<Box<dyn Event>>;
}

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