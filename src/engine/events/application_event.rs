use std::any::Any;
use crate::engine::events::EventType::{WindowClose, WindowResize};
use crate::engine::events::{Event, EventCategory, EventType};

pub struct WindowResizeEvent {
    pub width: u32,
    pub height: u32
}

impl Event for WindowResizeEvent {
    fn get_event_type(&self) -> EventType {
        WindowResize
    }
    fn get_name(&self) -> &str { "WindowResizeEvent"}

    fn get_category_flags(&self) -> EventCategory {
        EventCategory::Application
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn to_string(&self) -> String {
        format!("WindowResizeEvent: {}, {}", self.width, self.height)
    }
}

pub struct WindowCloseEvent;

impl Event for WindowCloseEvent {
    fn get_event_type(&self) -> EventType {
        WindowClose
    }

    fn get_name(&self) -> &str { "WindowCloseEvent"}

    fn get_category_flags(&self) -> EventCategory {
        EventCategory::Application
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

}

pub struct AppTickEvent;

impl Event for AppTickEvent {
    fn get_event_type(&self) -> EventType {
        EventType::AppTick
    }

    fn get_name(&self) -> &str { "AppTickEvent"}

    fn get_category_flags(&self) -> EventCategory {
        EventCategory::Application
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct AppUpdateEvent;

impl Event for AppUpdateEvent {
    fn get_event_type(&self) -> EventType {
        EventType::AppUpdate
    }

    fn get_name(&self) -> &str { "AppUpdateEvent"}

    fn get_category_flags(&self) -> EventCategory {
        EventCategory::Application
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct AppRenderEvent;

impl Event for AppRenderEvent {
    fn get_event_type(&self) -> EventType {
        EventType::AppRender
    }

    fn get_name(&self) -> &str { "AppRenderEvent"}

    fn get_category_flags(&self) -> EventCategory {
        EventCategory::Application
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}