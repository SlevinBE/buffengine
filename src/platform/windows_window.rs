use std::sync::mpsc::Receiver;
use glfw::{Glfw, WindowEvent, WindowMode};

pub struct WindowsWindow {
    glfw: Glfw,
    window_handle: glfw::PWindow,
    events: Receiver<(f64, WindowEvent)>
}

impl WindowsWindow {
    pub fn new(width: u32, height: u32, title: &str) -> WindowsWindow {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

        let (mut window, receiver) = glfw.create_window(width, height, title, WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

        window.set_framebuffer_size_polling(true);
        window.set_key_polling(true);

        WindowsWindow {
            glfw,
            window_handle: window,
            events
        }
    }
}