use log::info;
use crate::platform::windows_window::WindowsWindow;

pub struct Application {

}

impl Application {
    pub fn run(&self) {
        info!("Engine started");
        
        WindowsWindow::new(1024, 1024, "BuffEngine");
    }
}