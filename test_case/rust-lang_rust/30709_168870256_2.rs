 rust
// C.rs
#![crate_type = "lib"]
extern crate A;
extern crate B;

pub fn baz() {
    B::bar();
}
