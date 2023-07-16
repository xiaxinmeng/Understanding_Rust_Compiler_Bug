 rust
extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct Core {
    context: sdl2::Sdl,
}

impl Core {
    #[no_mangle]
    pub extern "C" fn new() -> Core {
        Core { context: sdl2::init().unwrap() }
    }
}
