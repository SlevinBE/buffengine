use crate::engine::core::key_codes::KeyCode;
use crate::engine::core::mouse_codes::MouseCode;
use crate::engine::core::window::{Window, WindowProps};
use crate::engine::events::application_event::{WindowCloseEvent, WindowResizeEvent};
use crate::engine::events::event::Event;
use crate::engine::events::event::EventType::WindowClose;
use crate::engine::events::key_event::{KeyPressedEvent, KeyReleasedEvent};
use crate::engine::events::mouse_event::{MouseButtonPressedEvent, MouseButtonReleasedEvent, MouseMovedEvent, MouseScrolledEvent};
use crate::engine::renderer::{Renderer, WgpuRenderer};
use crate::platform::input::WinitKeyCode;
use log::debug;
use std::sync::mpsc::{channel, Receiver, Sender};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::MouseScrollDelta::{LineDelta, PixelDelta};
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::PhysicalKey;
use winit::keyboard::PhysicalKey::Code;
use winit::keyboard::PhysicalKey::Unidentified;
use winit::window::WindowId;

type WinitWindow = winit::window::Window;

pub struct UniversalWindow<'window> {
    renderer: Option<WgpuRenderer<'window>>,
    events_sender: Sender<Box<dyn Event>>,
    events_receiver: Receiver<Box<dyn Event>>,
    window_data: WindowData
}

struct WindowData {
    title: String,
    width: u32,
    height: u32,
    vsync: bool
}

impl <'window> UniversalWindow<'window> {
    pub fn new(window_props: WindowProps) -> UniversalWindow<'window> {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);

        let (events_sender, events_receiver) = channel();
        
        let mut windows_window = UniversalWindow {
            renderer: None,
            events_sender,
            events_receiver,
            window_data: WindowData {
                title: window_props.title.clone(),
                width: window_props.width,
                height: window_props.height,
                vsync: false
            }   
        };

        event_loop.run_app(&mut windows_window);
        windows_window
    }

    fn map_event(&mut self, event: WindowEvent) -> Option<Box<dyn Event>> {
        match event {
            WindowEvent::CloseRequested => {
                Some(Box::new(WindowCloseEvent {}))
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
                self.update();
                None
            }
            (unknown_event) => {
                debug!("Unknown event {:?}", unknown_event);
                None
            }
        }
    }
}

impl ApplicationHandler for UniversalWindow<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = WinitWindow::default_attributes()
            .with_title(&self.window_data.title)
            .with_inner_size(LogicalSize::new(self.window_data.width, self.window_data.height));
        let window: WinitWindow = event_loop.create_window(window_attributes).unwrap();
        self.renderer = Some(WgpuRenderer::new(window));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        if let Some(event) = Self::map_event(self, event) {
            if event.get_event_type() == WindowClose {
                event_loop.exit();
            }
            self.events_sender.send(event);
        }
    }
}

impl <'window> Window for UniversalWindow<'window> {
    fn update(&mut self) {
        if let Some(renderer) = &self.renderer {
            renderer.draw();
        }
    }

    fn get_width(&self) -> u32 {
        self.window_data.width
    }

    fn get_height(&self) -> u32 {
        self.window_data.height
    }

    fn set_vsync(&mut self, enabled: bool) {
        // TODO: implement vsync
        self.window_data.vsync = enabled;
    }

    fn is_vsync_enabled(&self) -> bool {
        self.window_data.vsync
    }

    fn events(&self) -> &Receiver<Box<dyn Event>> {
        &self.events_receiver
    }

}