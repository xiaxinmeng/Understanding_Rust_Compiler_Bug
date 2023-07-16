rust
#![feature(nll)]
fn main() {
    let x: &mut [u32] = &mut [1, 2];
    x.split_at_mut(x.len() / 2);
}
