 rust
#[no_mangle]
pub extern fn reverse(s: *const libc::c_char) -> *mut RustAllocChar {
    let s = unsafe { CStr::from_ptr(s) };
    let s2 = s.to_str().unwrap();
    let s3: String = s2.chars().rev().collect();
    let s4 = CString::new(s3).unwrap();
    s4.into_ptr()
}

#[no_mangle]
pub extern fn cleanup(s: *mut RustAllocChar) {
    unsafe { CString::from_ptr(s) };
}
