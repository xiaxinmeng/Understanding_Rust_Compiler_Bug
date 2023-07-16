Rust
#![feature(box_syntax)]
fn main() {
    let x: Box<[isize]> = box { loop {} };
}
