rust
// src/test/ui/auxiliary/issue-60363.rs
#![crate_type = "rlib"]
#![feature(const_fn)]

use std::cell::UnsafeCell;

pub struct Handler {
    h: UnsafeCell<*const dyn FnMut()>,
}

impl Handler {
    pub const fn new() -> Self {
        Self {
            h: UnsafeCell::new(&Self::default_handler),
        }
    }

    fn default_handler() {}
}

unsafe impl Sync for Handler {}
