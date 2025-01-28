#![allow(dead_code,unused)]

use std::error::Error;
use buffengine::engine::application::Application;
use buffengine::engine::core::window::WindowProps;
use buffengine::logger;
use buffengine::platform::windows_window::WindowsWindow;
use buffengine::sample_game::debug_overlay::DebugOverlay;
use buffengine::sample_game::sample_layer::SampleLayer;
use logger::init_logging;

fn main() -> Result<(), Box<dyn Error>> {
    init_logging()?;

    let sample_layer = SampleLayer {
        name: String::from("Sample Layer")
    };
    let debug_overlay = DebugOverlay {
        name: String::from("Debug Overlay")
    };

    let window = Box::new(WindowsWindow::new(&WindowProps::default()));
    let mut app = Application::new(window);
    app.push_layer(Box::new(sample_layer));
    app.push_overlay(Box::new(debug_overlay));
    app.run();

    Ok(())
}
