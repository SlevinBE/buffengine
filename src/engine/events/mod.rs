use std::any::Any;
use bitmask_enum::bitmask;
use crate::engine::core::key_codes::KeyCode;
use crate::engine::core::mouse_codes::MouseCode;

pub mod event_handling;
pub mod winit_event_mapper;
pub mod winit_input_mapper;

#[derive(Debug)]
pub enum Event {
    ApplicationEvent(ApplicationEvent),
    MouseEvent(MouseEvent),
    KeyboardEvent(KeyboardEvent),
    UserEvent(Box<dyn Any>)
}

#[derive(Debug)]
pub enum ApplicationEvent {
    WindowClosed, 
    WindowResized {
        width: u32,
        height: u32
    }, 
    RenderRequested
}

#[derive(Debug)]
pub enum MouseEvent {
    MouseButtonPressed{
        button: MouseCode
    }, 
    MouseButtonReleased {
        button: MouseCode
    }, 
    MouseMoved {
        x: f64,
        y: f64   
    }, 
    MouseScrolled {
        x_offset: f64,
        y_offset: f64
    }
}

#[derive(Debug)]
pub enum KeyboardEvent {
    KeyPressed {
        key_code: KeyCode,
        is_repeat: bool
    }, 
    KeyReleased {
        key_code: KeyCode
    }
}