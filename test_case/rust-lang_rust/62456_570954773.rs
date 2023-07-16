rust
#![feature(const_generics)]

fn foo<const N: usize>() {
    let _ = [""; N + 1];
}
