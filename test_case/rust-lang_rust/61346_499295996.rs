rust
#![feature(const_generics)]

pub struct Array<T, const N: usize> { x: [T; N] }

fn main() {
    let _ = Array::<u32, 8> { x: [0u32; 8] };
}
