rust
#![feature(inline_const)]
fn main() {
    let _: [Vec<i32>; 4] = [const { Vec::<i32>::new() }; 4];
}
