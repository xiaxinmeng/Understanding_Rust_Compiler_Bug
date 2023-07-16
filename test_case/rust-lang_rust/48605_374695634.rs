rust
#![feature(nll)]

fn main() {
    let mut a = 5;
    let b = &mut a;
    let c = &mut *b;
}
