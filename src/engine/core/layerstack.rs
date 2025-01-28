use std::slice::Iter;
use crate::engine::core::layer::Layer;

pub struct LayerStack {
    layers: Vec<Box<dyn Layer>>,
    overlays: Vec<Box<dyn Layer>>
}

impl LayerStack {

    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            overlays: Vec::new()
        }
    }

    pub fn push_layer(&mut self, layer: Box<dyn Layer>) {
        self.layers.push(layer);
    }

    pub fn push_overlay(&mut self, overlay: Box<dyn Layer>) {
        self.overlays.push(overlay);
    }

    pub fn pop_layer(&mut self) {
        self.layers.pop();
    }

    pub fn pop_overlay(&mut self) {
        self.overlays.pop();
    }

    pub fn layers(&self) -> Iter<Box<dyn Layer>> {
        self.layers.iter()
    }

    pub fn overlays(&self) -> Iter<Box<dyn Layer>> {
        self.overlays.iter()
    }
}