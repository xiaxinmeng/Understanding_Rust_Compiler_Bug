rust
#![feature(const_generics)]

fn as_chunks<const N: usize>() -> [u8; N] {
    loop {}
}

fn main() {
    let [x, y] = as_chunks();
}
