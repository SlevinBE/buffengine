#![allow(dead_code,unused)]

use std::error::Error;
use buffengine::engine::application::Application;
use buffengine::engine::core::window::WindowProps;
use buffengine::logger;
use buffengine::platform::windows_window::WindowsWindow;
use logger::init_logging;

fn main() -> Result<(), Box<dyn Error>> {
    init_logging()?;

    let window = Box::new(WindowsWindow::new(&WindowProps::default()));
    let mut app = Application::new(window);
    app.run();

    Ok(())
}
