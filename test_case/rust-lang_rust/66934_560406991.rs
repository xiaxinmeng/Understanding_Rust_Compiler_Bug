rust
#![feature(slice_patterns)]

fn main() {
    let x = [()];
    let [y @ ..] = x;
}
