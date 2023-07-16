rust
#![feature(nll)]
fn main() {
    let x = &mut [1, 2][..];
    x.split_at_mut(x.len() / 2);
}
