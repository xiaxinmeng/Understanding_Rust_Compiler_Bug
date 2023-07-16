rust
pub fn with_new() -> Box<Pages> {
    Box::new(Pages([0u8; SIZE]))
}

pub fn with_box_syntax() -> Box<Pages> {
    box Pages([0u8; SIZE])
}

pub fn with_default() -> Box<Pages> {
    Default::default()
}

pub fn with_raw_alloc() -> Box<Pages> {
    use std::alloc::{alloc_zeroed, Layout, handle_alloc_error};
    let layout = Layout::new::<Pages>();
    let ptr = unsafe { alloc_zeroed(layout) };
    if ptr.is_null() {
        handle_alloc_error(layout);
    }
    unsafe { Box::from_raw(ptr as *mut Pages) }
}
