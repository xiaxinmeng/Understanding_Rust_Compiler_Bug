rust
#![feature(const_fn)]
const fn foo(x: usize) -> usize {
    x
}
fn main() {
    [0; foo(2)];
}
