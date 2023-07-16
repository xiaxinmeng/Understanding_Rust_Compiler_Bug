 rust
#![no_std]
#![crate_type = "lib"]

const Z: () = panic!("cheese");

const Y: () = unreachable!();

const X: () = unimplemented!();
