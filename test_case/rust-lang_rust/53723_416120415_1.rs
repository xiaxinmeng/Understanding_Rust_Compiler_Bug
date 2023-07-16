rust
#![feature(nll)]
fn main() {
    let mut x = [1, 2];
    (&mut x as &mut [_]).split_at_mut((&mut x as &mut [_]).len() / 2);
}
