trait Window {
    fn update();
    
    fn get_width() -> u32;
    
    fn get_height() -> u32;
    
    fn set_vsync(enabled: bool);
    
    fn is_vsync_enabled() -> bool;
    
    fn get_native_window();
}

struct WindowProps {
    title: String,
    width: u32,
    height: u32
}

impl Default for WindowProps {
    fn default() -> Self {
        Self {
            title: "BuffEngine".to_string(),
            width: 1280,
            height: 720
        }
    }
}