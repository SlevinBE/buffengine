use log::debug;
use winit::event::{ElementState, WindowEvent};
use winit::event::MouseScrollDelta::{LineDelta, PixelDelta};
use winit::keyboard::PhysicalKey::{Code, Unidentified};
use crate::engine::events::ApplicationEvent::{RenderRequested, WindowClosed, WindowResized};
use crate::engine::events::{ApplicationEvent, Event};
use crate::engine::events::KeyboardEvent::{KeyPressed, KeyReleased};
use crate::engine::events::MouseEvent::{MouseButtonPressed, MouseButtonReleased, MouseMoved, MouseScrolled};
use crate::engine::events::winit_input_mapper::WinitKeyCode;

pub fn map_event(event: WindowEvent) -> Option<Event> {
    match event {
        WindowEvent::CloseRequested => {
            Some(Event::ApplicationEvent(WindowClosed))
        },
        WindowEvent::Resized(physical_size) => {
            Some(Event::ApplicationEvent(WindowResized {
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
                            Some(Event::KeyboardEvent(KeyPressed {
                                key_code,
                                is_repeat: event.repeat,
                            }))
                        }
                        ElementState::Released => {
                            Some(Event::KeyboardEvent(KeyReleased {
                                key_code
                            }))
                        }
                    }
                }
            }
        }
        WindowEvent::CursorMoved { device_id, position } => {
            Some(Event::MouseEvent(MouseMoved {
                x: position.x,
                y: position.y
            }))
        }
        WindowEvent::MouseInput { device_id, state, button } => {
            button.try_into().map_or(None, |mouse_code|
                match state {
                    ElementState::Pressed => Some(Event::MouseEvent(MouseButtonPressed { button: mouse_code })),
                    ElementState::Released => Some(Event::MouseEvent(MouseButtonReleased { button: mouse_code })),
                    _ => None
                }
            )
        }
        WindowEvent::MouseWheel { device_id, delta, phase } => {
            match delta {
                LineDelta(x, y) => None,
                PixelDelta(position) => Some(Event::MouseEvent(MouseScrolled {
                    x_offset: position.x,
                    y_offset: position.y
                }))
            }
        }
        WindowEvent::RedrawRequested => {
            Some(Event::ApplicationEvent(RenderRequested))
        }
        (unknown_event) => {
            debug!("Unknown event {:?}", unknown_event);
            None
        }
    }
}