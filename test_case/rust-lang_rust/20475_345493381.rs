rust
use std::cmp;
use std::ffi::CString;
use std::mem;
use std::ptr;

unsafe fn c_function(buf: *mut u8, len: usize) {
    let data = b"hello\0";

    // buf may or may not end with \0 depending on whether there was room for it
    ptr::copy(data.as_ptr(), buf, cmp::min(len, data.len()));
}

fn call_the_thing() -> CString {
    unsafe {
        let mut buf = mem::uninitialized::<[u8; 256]>();
        c_function(buf.as_mut_ptr(), buf.len());
        let end = buf.iter().position(|ch| *ch == b'\0').unwrap_or(buf.len());
        CString::from_vec_unchecked(buf[..end].to_vec())
    }
}

fn main() {
    println!("{:?}", call_the_thing().as_bytes());
}
