rust
#![allow(unused_variables)]
struct Zeroes;
impl Into<&'static [usize; 3]> for Zeroes {
    fn into(self) -> &'static [usize; 3] { &[0; 3] }
}
impl Into<[usize; 3]> for Zeroes {
    fn into(self) -> [usize; 3] { [0; 3] }
}
fn main() {
    let [a, b, c] = Zeroes.into(); // Doesn't work: is this an `[usize; 3]` or `&[usize; 3]`?
    let [d, e, f]: [_; 3] = Zeroes.into(); // Works
    let [g, h, i]: &[_; 3] = Zeroes.into(); // Works
}
