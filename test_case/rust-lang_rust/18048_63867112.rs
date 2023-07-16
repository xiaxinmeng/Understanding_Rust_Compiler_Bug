 rust
// bar.rs
#![crate_type="lib"]
#![feature(associated_types)]

pub trait Bar {
    type T;
}
