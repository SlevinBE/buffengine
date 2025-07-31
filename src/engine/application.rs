use crate::engine::core::layer::Layer;
use crate::engine::core::layerstack::LayerStack;
use crate::engine::core::window::{WindowProps};
use crate::engine::events::{Event};
use log::info;
use std::cell::{Cell, Ref, RefCell};
use std::sync::mpsc::{channel, Receiver, Sender};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::WindowId;
use crate::engine::events::ApplicationEvent::{RenderRequested, WindowClosed};
use crate::engine::events::winit_event_mapper::map_event;
use crate::engine::renderer::{Renderer, Scene};
use crate::engine::renderer::wgpu::wgpu_renderer::WgpuRenderer;

type WinitWindow = winit::window::Window;

pub struct Application<'app> {
    layerstack: LayerStack,
    renderer: Option<WgpuRenderer<'app>>,
    events_sender: Sender<Event>,
    events_receiver: Receiver<Event>,
    window_props: WindowProps
}

impl <'app> Application<'app> {
    
    pub fn new(window_props: WindowProps) -> Application<'app> {
        let (events_sender, events_receiver) = channel();
        Self {
            layerstack: LayerStack::new(),
            renderer: None,
            events_sender,
            events_receiver,
            window_props
        }
    }
    
    pub fn run(&mut self) {
        info!("Engine started");

        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop.run_app(self);
    }

    pub fn push_layer(&mut self, layer: Box<dyn Layer>) {
        self.layerstack.push_layer(layer);
    }

    pub fn push_overlay(&mut self, overlay: Box<dyn Layer>) {
        self.layerstack.push_overlay(overlay);
    }

    fn process_events(&mut self, event_loop: &ActiveEventLoop) {
        while let Ok(event) = self.events_receiver.try_recv() {
            info!("Event: {:?}", event);
            
            match event {
                Event::ApplicationEvent(WindowClosed) => self.on_window_closed(event_loop),
                Event::ApplicationEvent(RenderRequested) => self.on_app_render(),
                _ => {
                    // ignore for now
                }
            }

            let mut event_handled = false;
            for overlay in self.layerstack.overlays().rev() {
                if !event_handled {
                    event_handled = overlay.handle_event(&event)
                }
            }
            for layers in self.layerstack.layers().rev() {
                if !event_handled {
                    event_handled = layers.handle_event(&event)
                }
            }
        }
    }

    fn update_layers(&mut self) {
        for layer in self.layerstack.layers() {
            layer.update()
        }
        for overlay in self.layerstack.overlays() {
            overlay.update()
        }
    }

    fn run_renderer(&mut self) {
        if let Some(renderer) = &mut self.renderer {
            for layer in self.layerstack.layers() {
                if let Some(scene) = layer.as_scene() {
                    renderer.render(scene)
                }
            }
            for overlay in self.layerstack.overlays() {
                if let Some(scene) = overlay.as_scene() {
                    renderer.render(scene)
                }
            }
        }
    }

    fn on_app_render(&mut self) {
        self.update_layers();
        self.run_renderer();
    }

    fn on_window_closed(&self, event_loop: &ActiveEventLoop) {
        event_loop.exit();
    }
}

impl <'app> ApplicationHandler for Application<'app> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = WinitWindow::default_attributes()
            .with_title(&self.window_props.title)
            .with_inner_size(LogicalSize::new(
                self.window_props.width, 
                self.window_props.height
            ));
        let window: WinitWindow = event_loop.create_window(window_attributes).unwrap();
        self.renderer = Some(WgpuRenderer::new(window));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        if let Some(event) = map_event(event) {
            self.events_sender.send(event);
            self.process_events(event_loop);
        }
    }
}