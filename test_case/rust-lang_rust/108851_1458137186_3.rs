rust
#![feature(inline_const)]

fn foo<const N: usize>() {
    const { assert!(N % 2 == 0, "N must be even") }
}

fn main() {
    foo::<2>(); // OK
    foo::<3>(); // ERROR
}
