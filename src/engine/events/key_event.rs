use crate::engine::core::key_codes::KeyCode;
use crate::engine::events::event::{Event, EventCategory, EventType};

pub trait KeyEvent : Event {
    fn get_key_code(&self) -> &KeyCode;
}


pub struct KeyPressedEvent {
    pub key_code: KeyCode,
    pub is_repeat: bool
}

impl KeyEvent for KeyPressedEvent {
    fn get_key_code(&self) -> &KeyCode {
        &self.key_code
    }
}

impl Event for KeyPressedEvent {
    fn get_category_flags(&self) -> EventCategory {
        EventCategory::Keyboard | EventCategory::Input
    }
    fn get_event_type(&self) -> EventType {
        EventType::KeyPressed
    }
    fn get_name(&self) -> &str {
        "KeyPressed"
    }
    fn to_string(&self) -> String {
        format!("KeyPressedEvent: {:?}, {}", self.key_code, self.is_repeat)
    }
}

pub struct KeyReleasedEvent {
    pub key_code: KeyCode
}

impl KeyEvent for KeyReleasedEvent {
    fn get_key_code(&self) -> &KeyCode {
        &self.key_code
    }
}

impl Event for KeyReleasedEvent {
    fn get_category_flags(&self) -> EventCategory {
        EventCategory::Keyboard | EventCategory::Input
    }
    fn get_event_type(&self) -> EventType {
        EventType::KeyReleased
    }
    fn get_name(&self) -> &str {"KeyReleased"}
    fn to_string(&self) -> String {format!("KeyReleasedEvent: {:?}", self.key_code)}
}

struct KeyTypedEvent {
    pub key_code: KeyCode
}

impl KeyEvent for KeyTypedEvent {
    fn get_key_code(&self) -> &KeyCode {
        &self.key_code
    }
}

impl Event for KeyTypedEvent {
    fn get_category_flags(&self) -> EventCategory {
        EventCategory::Keyboard | EventCategory::Input
    }
    fn get_event_type(&self) -> EventType {
        EventType::KeyTyped
    }
    fn get_name(&self) -> &str { "KeyTyped" }
    fn to_string(&self) -> String { format!("KeyTypedEvent: {:?}", self.key_code) }
}