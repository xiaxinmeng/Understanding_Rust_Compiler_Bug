rust
#![allow(incomplete_features)]
#![feature(inline_const)]
fn main() {
    const N: u32 = 10;

    let range = 1 .. const { N - 1 };
    println!("{:?}", range);
}
