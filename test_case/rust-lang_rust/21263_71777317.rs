 rust
#![no_std]
extern crate core;

mod std {
    pub use core::ops;
}
