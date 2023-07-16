rust
#![feature(const_generics)]

pub struct Array<T, const N: usize>([T; N]);

fn main() {
    let _ = Array([0u32; 8]);
}
