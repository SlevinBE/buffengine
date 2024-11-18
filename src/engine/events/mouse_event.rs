use EventType::MouseButtonPressed;
use crate::engine::core::mouse_codes::MouseCode;
use crate::engine::events::event::{Event, EventCategory, EventType};
use crate::engine::events::event::EventType::{MouseButtonReleased, MouseMoved, MouseScrolled};

pub struct MouseMovedEvent {
    pub x: f32,
    pub y: f32
}

impl Event for MouseMovedEvent {
    fn get_event_type(&self) -> EventType {
        MouseMoved
    }

    fn get_name(&self) -> &str { "MouseMoved" }

    fn get_category_flags(&self) -> EventCategory {
        EventCategory::Mouse | EventCategory::Input
    }

    fn to_string(&self) -> String {
        format!("MouseMovedEvent: {}, {}", self.x, self.y)
    }
}

pub struct MouseScrolledEvent {
    pub x_offset: f32,
    pub y_offset: f32
}

impl Event for MouseScrolledEvent {
    fn get_event_type(&self) -> EventType {
        MouseScrolled
    }

    fn get_name(&self) -> &str {
        "MouseScrolled"
    }

    fn get_category_flags(&self) -> EventCategory {
        EventCategory::Mouse | EventCategory::Input
    }

    fn to_string(&self) -> String {
        format!("MouseScrolledEvent: {},{}", self.x_offset, self.y_offset)
    }
}

pub trait MouseButtonEvent : Event {

    fn get_mouse_button(&self) -> &MouseCode;

}

pub struct MouseButtonPressedEvent {
    pub button: MouseCode
}

impl MouseButtonEvent for MouseButtonPressedEvent {
    fn get_mouse_button(&self) -> &MouseCode {
        &self.button
    }
}

impl Event for MouseButtonPressedEvent {
    fn get_event_type(&self) -> EventType { MouseButtonPressed }
    fn get_name(&self) -> &str { "MouseButtonPressed" }

    fn get_category_flags(&self) -> EventCategory {
        EventCategory::Mouse | EventCategory::Input | EventCategory::MouseButton
    }

    fn to_string(&self) -> String {
        format!("MouseButtonPressedEvent: {:?}", &self.button)
    }
}

pub struct MouseButtonReleasedEvent {
    pub button: MouseCode
}

impl MouseButtonEvent for MouseButtonReleasedEvent {
    fn get_mouse_button(&self) -> &MouseCode {
        &self.button
    }
}

impl Event for MouseButtonReleasedEvent {
    fn get_event_type(&self) -> EventType {
        MouseButtonReleased
    }

    fn get_name(&self) -> &str {
        "MouseButtonReleased"
    }

    fn get_category_flags(&self) -> EventCategory {
        EventCategory::Mouse | EventCategory::Input | EventCategory::MouseButton
    }

    fn to_string(&self) -> String {
        format!("MouseButtonReleasedEvent: {:?}", &self.button)
    }
}