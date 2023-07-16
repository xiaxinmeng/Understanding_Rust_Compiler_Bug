rust
use std::mem;
use std::os::raw::c_void;

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::<_>::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    // ptr as *mut (); // uncomment here
    ptr as *mut c_void
}

fn main() {
    println!("{:p}", alloc(10));
}
