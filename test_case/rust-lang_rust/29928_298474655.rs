rust
#![feature(const_fn)]
const fn f(x: usize) -> A {
    A { field: x }
}
struct A {
    field: usize,
}
fn main() {
    let _ = [0; f(5).field];
}
