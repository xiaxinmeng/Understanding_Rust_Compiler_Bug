rust
extern "C" {
    // N.B., mutability can be easily incorrect in FFI calls -- as
    // in C, the default is mutable pointers.
    fn ffi(c: *mut u8);
    fn int_ffi(c: *mut i32);
}
ffi(a.as_ptr() as *mut _);
int_ffi(num as *const _ as *mut _);
int_ffi(&3 as *const _ as *mut _);
