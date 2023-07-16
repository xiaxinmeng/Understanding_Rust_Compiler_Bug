rust
use std::ffi::CString;

#[inline(never)]
fn f(c_str: *const i8) {
    println!("{:?}", c_str);
}

fn main() {
    let c_string = CString::new("Hello there").unwrap();
    f(c_string.as_ptr());
}
