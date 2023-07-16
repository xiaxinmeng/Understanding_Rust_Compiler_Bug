rust
#![crate_type = "lib"]
use std::mem::transmute;
fn g<T>(a: [[T; 2]; 2]) -> [T; 4] {
    unsafe { transmute(a) }
}
