Rust
use std::mem::MaybeUninit;

fn main() {
    struct Foo {a: u32, b: bool}
    struct FooInitHelper { a: MaybeUninit<u32>, b: MaybeUninit<bool> };
    let mut idk = FooInitHelper { a: MaybeUninit::uninit(), b: MaybeUninit::uninit() };
    // Initialize the values here
    let mut idk: Foo = std::mem::transmute(idk);
}
