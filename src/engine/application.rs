use std::cell::{Cell, Ref, RefCell};
use log::info;
use crate::engine::core::layer::Layer;
use crate::engine::core::layerstack::LayerStack;
use crate::engine::core::window::{Window, WindowProps};
use crate::engine::events::event::EventType;
use crate::platform::windows_window::UniversalWindow;

pub struct Application {
    window: Box<dyn Window>,
    is_running: Cell<bool>,
    layerstack: LayerStack
}

impl Application {
    
    pub fn new(window: Box<dyn Window>) -> Application {
        Self {
            window,
            is_running: Cell::new(true),
            layerstack: LayerStack::new()
        }
    }
    
    pub fn run(&mut self) {
        info!("Engine started");
    
        while self.is_running.get() {
            for layer in self.layerstack.layers() {
                layer.on_update()
            }
            for overlay in self.layerstack.overlays() {
                overlay.on_update()
            }

            self.window.update();
            self.process_events();
        }
    }

    pub fn push_layer(&mut self, layer: Box<dyn Layer>) {
        self.layerstack.push_layer(layer);
    }

    pub fn push_overlay(&mut self, overlay: Box<dyn Layer>) {
        self.layerstack.push_overlay(overlay);
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

            let mut event_handled = false;
            for overlay in self.layerstack.overlays().rev() {
                if !event_handled {
                    event_handled = overlay.on_event(&event)
                }
            }
            for layers in self.layerstack.layers().rev() {
                if !event_handled {
                    event_handled = layers.on_event(&event)
                }
            }
        }
    }

    fn on_window_closed(&self) {
        self.is_running.set(false);
    }
}