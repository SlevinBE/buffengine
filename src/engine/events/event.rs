use bitmask_enum::bitmask;

pub enum EventType {
    WindowClose, WindowResize, WindowFocus, WindowLostFocus, WindowMoved,
    AppTick, AppUpdate, AppRender,
    KeyPressed, KeyReleased, KeyTyped,
    MouseButtonPressed, MouseButtonReleased, MouseMoved, MouseScrolled
}

#[bitmask(u8)]
pub enum EventCategory {
    Application,
    Input,
    Keyboard,
    Mouse,
    MouseButton,
}

pub trait Event {
    fn get_event_type(&self) -> EventType;

    fn get_name(&self) -> &str;

    fn get_category_flags(&self) -> EventCategory;

    fn is_in_category(&self, category: EventCategory) -> bool {
        self.get_category_flags().contains(category)
    }

    fn to_string(&self) -> String {
        self.get_name().to_string()
    }

}