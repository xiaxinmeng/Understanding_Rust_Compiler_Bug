rust
#![feature(const_generics)]

pub struct Array<T, const N: usize> {
    data: [T; {N}]
}

fn main() {
    let _ = Array { data: [0u32; 8] };
}
