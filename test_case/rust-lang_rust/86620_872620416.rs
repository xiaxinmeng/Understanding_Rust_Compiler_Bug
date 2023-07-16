rust
// src/lib.rs
use ppv_lite86::*;

pub struct S;

// ppv_lite86/src/lib.rs
pub trait VZip {
    fn vzip() -> usize;
}

impl<T> VZip for T {
    fn vzip() -> usize {
        0
    }
}
