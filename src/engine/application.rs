use std::cell::{Cell, Ref, RefCell};
use log::info;
use crate::engine::core::window::{Window, WindowProps};
use crate::platform::windows_window::WindowsWindow;

pub struct Application {
    window: Box<dyn Window>,
    is_running: Cell<bool>
}

impl Application {
    
    pub fn new(window: Box<dyn Window>) -> Application {
        Self {
            window,
            is_running: Cell::new(true)
        }
    }
    
    pub fn run(&mut self) {
        info!("Engine started");
    
        while self.is_running.get() {
            self.window.update();
            self.is_running.set(!self.window.is_closing());
        }
    }
}