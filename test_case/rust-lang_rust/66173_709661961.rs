 rust
#[macro_use]
extern crate objc;

use std::ptr;
use objc::runtime::Object;

fn main() {
    let obj: *mut Object = ptr::null_mut();
    unsafe {
        msg_send![obj, retain];
    };
}
