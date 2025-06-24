#![allow(dead_code,unused)]

use buffengine::engine::application::Application;
use buffengine::engine::core::window::WindowProps;
use buffengine::logger;
use buffengine::sample_game::debug_overlay::DebugOverlay;
use buffengine::sample_game::scene_layer::SceneLayer;
use logger::init_logging;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    init_logging()?;

    let sample_layer = SceneLayer::new(String::from("Sample Layer"));
    let debug_overlay = DebugOverlay {
        name: String::from("Debug Overlay")
    };

    let window_props = WindowProps::default();
    let mut app = Application::new(window_props);
    app.push_layer(Box::new(sample_layer));
    app.push_overlay(Box::new(debug_overlay));
    app.run();

    Ok(())
}
