rust
#![feature(const_generics)]
#![feature(const_evaluatable_checked)]

fn foo<F: FnOnce([u8; N - 1]), const N: usize>(val: F) {}

fn main() {
    foo(|arg: [u8; 5]| ());
}
