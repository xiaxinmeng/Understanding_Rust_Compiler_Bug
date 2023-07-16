 rust
use std::ffi::CString;

fn main() {
    let a = "1";
    let b = "2";
    let a_ptr = CString::from_slice(a.as_bytes()).as_ptr();
    let b_ptr = CString::from_slice(b.as_bytes()).as_ptr();

    println!("{:?}", a_ptr);
    println!("{:?}", b_ptr);
}
