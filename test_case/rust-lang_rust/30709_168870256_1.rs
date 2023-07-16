 rust
// B.rs
#![crate_type = "lib"]
extern crate A;

pub fn bar() {
    A::foo()
}
