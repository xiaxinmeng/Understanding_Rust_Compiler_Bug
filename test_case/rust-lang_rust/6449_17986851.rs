 rust
pub enum MouseButton {
    MOUSE_BUTTON_1 = 1,
    MOUSE_BUTTON_2 = 2,
    MOUSE_BUTTON_3 = 3,
    MOUSE_BUTTON_4 = 4,
    MOUSE_BUTTON_5 = 5,
    MOUSE_BUTTON_6 = 6,
    MOUSE_BUTTON_7 = 7,
    MOUSE_BUTTON_8 = 8,
}

pub static MOUSE_BUTTON_LEFT    : MouseButton = MOUSE_BUTTON_1;
pub static MOUSE_BUTTON_RIGHT   : MouseButton = MOUSE_BUTTON_2;
pub static MOUSE_BUTTON_MIDDLE  : MouseButton = MOUSE_BUTTON_3;

fn mouse_button_to_str(button: MouseButton) -> ~str {
    match button {
        MOUSE_BUTTON_LEFT     => ~"Left",
        MOUSE_BUTTON_RIGHT    => ~"Right",
        MOUSE_BUTTON_MIDDLE   => ~"Middle",
        MOUSE_BUTTON_4        => ~"4",
        MOUSE_BUTTON_5        => ~"5",
        MOUSE_BUTTON_6        => ~"6",
        MOUSE_BUTTON_7        => ~"7",
        MOUSE_BUTTON_8        => ~"8",
    }
}

fn main() {}
