rust
extern "C" {
    pub fn realloc(p: *mut std::ffi::c_void, size: usize) -> *mut std::ffi::c_void;
}

pub fn foo(s: usize) -> *mut u8 {
    unsafe {
        realloc(std::ptr::null_mut(), s) as *mut u8
    }
}
