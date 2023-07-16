rust
#![feature(extern_types)]
#![crate_type="lib"]

extern "C" {
    type c_void;

    fn malloc(n: usize) -> *mut c_void;
}

#[no_mangle]
pub fn call_malloc() {
    unsafe { malloc(1); }
}
