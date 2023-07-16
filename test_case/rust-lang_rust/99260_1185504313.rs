rust
#![feature(c_unwind)]

extern "C-unwind" {
    fn happy();
    fn unfortunate();
}

#[inline(never)]
#[no_mangle]
pub fn notable_function() -> i32 {
    unsafe {
        std::panic::catch_unwind(|| {
            happy(); 42
        }).unwrap_or_else(|e| {
            std::mem::forget(e);
            unfortunate();
            13
        })
    }
}
