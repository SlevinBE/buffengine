use MouseCode::{Button0, Button1, Button2, Button7};

#[derive(Debug)]
pub enum MouseCode {
    Button0                = 0,
    Button1                = 1,
    Button2                = 2,
    Button3                = 3,
    Button4                = 4,
    Button5                = 5,
    Button6                = 6,
    Button7                = 7,
}

impl MouseCode {
    const BUTTON_LAST: MouseCode = Button7;
    const BUTTON_LEFT: MouseCode = Button0;
    const BUTTON_RIGHT: MouseCode = Button1;
    const BUTTON_MIDDLE: MouseCode = Button2;
}