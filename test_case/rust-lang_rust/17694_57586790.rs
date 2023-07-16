 rust
#![feature(unboxed_closures)]

fn main() {
    let x = 123i;
    let _ = |&: | x;
}
