
use std::borrow::{Borrow, Cow};
use std::ffi::{CStr, CString};

fn main() {
    let x = CString::new("x").unwrap();
    let y: Cow<CStr> = x.borrow().into();
}
