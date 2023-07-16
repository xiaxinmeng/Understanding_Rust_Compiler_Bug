rust
#![feature(const_indexing)]
fn main() {
    const A: [usize; 1] = [0; 1];
    let b = [0; A[0]];
}
