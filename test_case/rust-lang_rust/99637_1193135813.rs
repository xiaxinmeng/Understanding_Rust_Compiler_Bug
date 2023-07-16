rust
// foo.rs
#![no_std]
pub fn foo() {
    bar::x();
    let b = b"hello";
    let _v = b.to_vec();
}
