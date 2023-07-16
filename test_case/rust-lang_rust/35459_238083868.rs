 rust
use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn kernel(s: *const c_char) -> i32 {
    let cs = unsafe { CStr::from_ptr(s) };
    let s = cs.to_string_lossy();
    println!("arg string [{}] len [{}]", s, s.len());
    42
}
