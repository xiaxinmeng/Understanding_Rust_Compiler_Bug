rust
#![feature(nll)]

fn main() {
    let mut v: Vec<()> = Vec::new();
    || &mut v;
}
