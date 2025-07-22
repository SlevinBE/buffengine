use std::any::Any;
use bitmask_enum::bitmask;

pub mod event_handling;
pub mod mouse_event;
pub mod application_event;
pub mod key_event;
pub mod winit_event_mapper;
pub mod winit_input_mapper;

pub trait Event: Any {
    fn get_event_type(&self) -> EventType;

    fn get_name(&self) -> &str;

    fn get_category_flags(&self) -> EventCategory;

    fn is_in_category(&self, category: EventCategory) -> bool {
        self.get_category_flags().contains(category)
    }
    
    fn as_any(&self) -> &dyn Any;

    fn to_string(&self) -> String {
        self.get_name().to_string()
    }

}

#[derive(Hash, PartialEq, Eq, Debug)]
pub enum EventType {
    WindowClose, WindowResize, WindowFocus, WindowLostFocus, WindowMoved,
    AppTick, AppUpdate, AppRender,
    KeyPressed, KeyReleased, KeyTyped,
    MouseButtonPressed, MouseButtonReleased, MouseMoved, MouseScrolled
}

#[bitmask(u8)]
pub enum EventCategory {
    Application,
    Input,
    Keyboard,
    Mouse,
    MouseButton,
}