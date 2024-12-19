use crate::engine::core::key_codes::KeyCode;
use crate::engine::core::mouse_codes::MouseCode;
use crate::engine::core::window::{Window, WindowProps};
use crate::engine::events::application_event::{WindowCloseEvent, WindowResizeEvent};
use crate::engine::events::event::Event;
use crate::engine::events::event::EventType::WindowClose;
use crate::engine::events::key_event::{KeyPressedEvent, KeyReleasedEvent};
use crate::engine::events::mouse_event::{MouseButtonPressedEvent, MouseButtonReleasedEvent, MouseMovedEvent, MouseScrolledEvent};
use glfw::{Action, Context, Glfw, GlfwReceiver, Key, MouseButton, SwapInterval, WindowEvent, WindowMode};
use log::debug;
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct WindowsWindow {
    glfw: Glfw,
    window_handle: glfw::PWindow,
    glfw_events: GlfwReceiver<(f64, WindowEvent)>,
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

impl WindowsWindow {
    pub fn new(window_props: &WindowProps) -> WindowsWindow {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        let (mut window, receiver) = glfw.create_window(
            window_props.width, window_props.height, &window_props.title, WindowMode::Windowed
        ).expect("Failed to create GLFW window.");

        window.set_framebuffer_size_polling(true);
        window.set_size_polling(true);
        window.set_key_polling(true);
        window.set_mouse_button_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_close_polling(true);
        window.set_scroll_polling(true);
        window.make_current();

        let (events_sender, events_receiver) = channel();
        
        WindowsWindow {
            glfw,
            window_handle: window,
            glfw_events: receiver,
            events_sender,
            events_receiver,
            window_data: WindowData {
                title: window_props.title.clone(),
                width: window_props.width,
                height: window_props.height,
                vsync: false
            }   
        }
    }

    fn process_events(&mut self) {
        self.glfw.poll_events();

        while let Some((_, window_event)) = self.glfw_events.receive() {
            if let Some(event) = self.map_event(window_event) {
                self.events_sender.send(event);
            }
        }
    }

    fn map_event(&self, window_event: WindowEvent) -> Option<Box<dyn Event>> {
        match window_event {
            WindowEvent::Close => Some(Box::new(WindowCloseEvent{})),
            WindowEvent::Size(width, height) => {
                Some(Box::new(WindowResizeEvent{
                    width: width as u32,
                    height: height as u32
                }))
            }
            WindowEvent::FramebufferSize(width, height) => {
                Some(Box::new(WindowResizeEvent{
                    width: width as u32,
                    height: height as u32
                }))
            }
            WindowEvent::Key(key, scancode, action, modifiers) => {
                match key.try_into() {
                    Err(_) => {
                        log::warn!("unable to map key code: {:?}", key);
                        None
                    }
                    Ok(key_code) => {
                        match action {
                            Action::Press => {
                                Some(Box::new(KeyPressedEvent {
                                    key_code,
                                    is_repeat: false,
                                }))
                            }
                            Action::Release => {
                                Some(Box::new(KeyReleasedEvent {
                                    key_code
                                }))
                            }
                            Action::Repeat => {
                                Some(Box::new(KeyPressedEvent {
                                    key_code,
                                    is_repeat: true,
                                }))
                            }
                        }
                    }
                }
            }
            WindowEvent::CursorPos(x_pos, y_pos) => {
                Some(Box::new(MouseMovedEvent{
                    x: x_pos,
                    y: y_pos
                }))
            }
            WindowEvent::MouseButton(button, action, modifiers) => {
                let mouse_code: MouseCode = button.into();

                match action {
                    Action::Press => Some(Box::new(MouseButtonPressedEvent{ button: mouse_code })),
                    Action::Release => Some(Box::new(MouseButtonReleasedEvent{ button: mouse_code })),
                    _ => None
                }
            }
            WindowEvent::Scroll(x_offset, y_offset) => {
                Some(Box::new(MouseScrolledEvent{
                    x_offset,
                    y_offset
                }))
            }
            (unknown_event) => {
                debug!("Unknown event {:?}", unknown_event);
                None
            }
        }
    }
}

impl Window for WindowsWindow {
    fn update(&mut self) {
        self.process_events();
        self.window_handle.swap_buffers();
    }

    fn get_width(&self) -> u32 {
        self.window_data.width
    }

    fn get_height(&self) -> u32 {
        self.window_data.height
    }

    fn set_vsync(&mut self, enabled: bool) {
        if enabled {
            self.glfw.set_swap_interval(SwapInterval::Sync(1));
        } else {
            self.glfw.set_swap_interval(SwapInterval::None);
        }
        
        self.window_data.vsync = enabled;
    }

    fn is_vsync_enabled(&self) -> bool {
        self.window_data.vsync
    }
    
    fn is_closing(&self) -> bool {
        self.window_handle.should_close()
    }

    fn events(&self) -> &Receiver<Box<dyn Event>> {
        &self.events_receiver
    }

}

/// conversion between KeyCode and Key. Integer key codes should match on both ends.
impl Into<KeyCode> for Key {
    fn into(self) -> KeyCode {
        unsafe {
            std::mem::transmute(self)
        }
    }
}

impl Into<MouseCode> for MouseButton {
    fn into(self) -> MouseCode {
        match self {
            MouseButton::Button1 => MouseCode::Button0,
            MouseButton::Button2 => MouseCode::Button1,
            MouseButton::Button3 => MouseCode::Button2,
            MouseButton::Button4 => MouseCode::Button3,
            MouseButton::Button5 => MouseCode::Button4,
            MouseButton::Button6 => MouseCode::Button5,
            MouseButton::Button7 => MouseCode::Button6,
            MouseButton::Button8 => MouseCode::Button7,
        }
    }
}