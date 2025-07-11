use crate::engine::core::scene::Scene;
use crate::engine::events::Event;
use crate::engine::gameobjects::GameObject;

pub trait Layer {
    fn update(&self);

    /// Called when an event is triggered.
    ///
    /// # Arguments
    ///
    /// * `event` - The event that needs to be handled
    ///
    /// # Returns
    ///
    /// * `bool` - Whether the event was handled.
    fn handle_event(&self, event: &Box<dyn Event>) -> bool;
    
    fn get_name(&self) -> &str;
    
    fn as_scene(&self) -> Option<&dyn Scene> {
        None
    }
}