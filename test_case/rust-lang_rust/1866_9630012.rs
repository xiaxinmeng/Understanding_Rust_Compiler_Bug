
mod llt {
    pub enum SDL_Window {}
}

#[link_name = "SDL2"]
extern mod ll {
    use llt::*;
    pub fn SDL_CreateWindow(title: *c_char, x: c_int, y: c_int,
                            w: c_int, h: c_int, flags: uint32_t) -> *SDL_Window;
}
