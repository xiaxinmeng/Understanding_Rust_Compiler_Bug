rust
#[derive(Copy, Clone)]
pub struct Glfw;

static mut GLFW: Option<Glfw> = None;
pub fn new() -> Glfw {
    unsafe { if let Some(glfw) = GLFW {
        return glfw;
    } else {
        todo!()
    }};
}
