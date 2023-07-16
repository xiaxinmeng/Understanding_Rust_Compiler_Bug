
    extern { fn __h_errno_location() -> *mut i32; }
    unsafe { *__h_errno_location() = 0 }
