use std::cell::{Cell, Ref, RefCell};
use log::info;
use crate::engine::core::window::{Window, WindowProps};
use crate::engine::events::event::EventType;
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
            self.process_events();
        }
    }

    fn process_events(&mut self) {
        while let Ok(event) = self.window.events().try_recv() {
            info!("Event: {:?}", event.get_event_type());
            
            match event.get_event_type() {
                EventType::WindowClose => self.on_window_closed(),
                _ => {
                    // ignore for now
                }
            }
        }
    }

    fn on_window_closed(&self) {
        self.is_running.set(false);
    }
}