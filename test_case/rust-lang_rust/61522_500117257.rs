 Rust
#![feature(const_generics)]

pub struct MyArray<const COUNT: usize>([u8; COUNT + 1]);

impl<const COUNT: usize> MyArray<{COUNT}> {
    fn inner(&self) -> &[u8; COUNT + 1] {
        &self.0
    }
}

fn main() {
}
