use std::sync::mpsc::Receiver;
use glfw::{Context, Glfw, GlfwReceiver, SwapInterval, WindowEvent, WindowMode};
use crate::engine::core::window::{Window, WindowProps};

pub struct WindowsWindow {
    glfw: Glfw,
    window_handle: glfw::PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
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
        window.set_key_polling(true);
        window.make_current();
        
        WindowsWindow {
            glfw,
            window_handle: window,
            events: receiver,
            window_data: WindowData {
                title: window_props.title.clone(),
                width: window_props.width,
                height: window_props.height,
                vsync: false
            }   
        }
    }
}

impl Window for WindowsWindow {
    fn update(&mut self) {
        self.glfw.poll_events();
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

}