
#![feature(const_generics)]

pub struct Array<const N: usize> {
    buffer: [i32; N],
}

impl<const N: usize> Array<{N}> {
    pub fn new() -> Self {
        Self {
            buffer: [0i32; N],
        }
    }
}

fn main () {
    let v: Array<5> = Array::new();
}
