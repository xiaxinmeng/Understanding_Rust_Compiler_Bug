rust
#![feature(const_fn_trait_bound, const_trait_impl)]
#![allow(dead_code)]

const fn foo<T: ~const Default + Copy>() -> T {
    T::default()
}

fn main() {
    const X1: u64 = foo(); // OK
    const X2: (u8, u64) = foo(); // Err
}
