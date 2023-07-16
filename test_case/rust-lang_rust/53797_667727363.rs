rust
#![deny(rust_2018_compatibility)]
extern crate ptr;

pub mod foo {
    pub use std::ptr;
    pub use ptr::Shared;
}
