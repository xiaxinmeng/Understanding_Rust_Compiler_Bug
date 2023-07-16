rust
use std::ptr;
fn main() {
    let _x = Box::new(1);
}
#[no_mangle]
pub unsafe extern "C" fn __rust_alloc(size: usize, align: usize) -> *mut u8 {
    eprintln!("hello");
    ptr::null_mut()
}
