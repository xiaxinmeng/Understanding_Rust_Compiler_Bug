rust
> extern "system" {
>   pub fn SetWindowLongPtrW(hwnd: *mut c_void, index: c_int, new_long: *mut c_void) -> *mut c_void
> }
> 