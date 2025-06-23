use log::debug;
use winit::event::{ElementState, WindowEvent};
use winit::event::MouseScrollDelta::{LineDelta, PixelDelta};
use winit::keyboard::PhysicalKey::{Code, Unidentified};
use crate::engine::events::application_event::{AppRenderEvent, WindowResizeEvent};
use crate::engine::events::Event;
use crate::engine::events::key_event::{KeyPressedEvent, KeyReleasedEvent};
use crate::engine::events::mouse_event::{MouseButtonPressedEvent, MouseButtonReleasedEvent, MouseMovedEvent, MouseScrolledEvent};
use crate::engine::events::winit_input_mapper::WinitKeyCode;

pub fn map_event(event: WindowEvent) -> Option<Box<dyn Event>> {
    match event {
        WindowEvent::CloseRequested => {
            Some(Box::new(crate::engine::events::application_event::WindowCloseEvent {}))
        },
        WindowEvent::Resized(physical_size) => {
            Some(Box::new(WindowResizeEvent {
                width: physical_size.width,
                height: physical_size.height
            }))
        }
        WindowEvent::KeyboardInput { device_id, event, .. } => {
            let key_result = match event.physical_key {
                Code(key_code) => (key_code as WinitKeyCode).try_into(),
                Unidentified(native_key_code) =>
                    Err(format!("unidentified key code: {:?}", native_key_code))
            };

            match key_result {
                Err(message) => {
                    log::warn!("{}", message);
                    None
                }
                Ok(key_code) => {
                    match event.state {
                        ElementState::Pressed => {
                            Some(Box::new(KeyPressedEvent {
                                key_code,
                                is_repeat: event.repeat,
                            }))
                        }
                        ElementState::Released => {
                            Some(Box::new(KeyReleasedEvent {
                                key_code
                            }))
                        }
                    }
                }
            }
        }
        WindowEvent::CursorMoved { device_id, position } => {
            Some(Box::new(MouseMovedEvent {
                x: position.x,
                y: position.y
            }))
        }
        WindowEvent::MouseInput { device_id, state, button } => {
            button.try_into().map_or(None, |mouse_code|
                match state {
                    ElementState::Pressed => Some(Box::new(MouseButtonPressedEvent { button: mouse_code })),
                    ElementState::Released => Some(Box::new(MouseButtonReleasedEvent { button: mouse_code })),
                    _ => None
                }
            )
        }
        WindowEvent::MouseWheel { device_id, delta, phase } => {
            match delta {
                LineDelta(x, y) => None,
                PixelDelta(position) => Some(Box::new(MouseScrolledEvent {
                    x_offset: position.x,
                    y_offset: position.y
                }))
            }
        }
        WindowEvent::RedrawRequested => {
            Some(Box::new(AppRenderEvent))
        }
        (unknown_event) => {
            debug!("Unknown event {:?}", unknown_event);
            None
        }
    }
}