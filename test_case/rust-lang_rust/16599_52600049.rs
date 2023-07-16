 rust
#![feature(unboxed_closures, unboxed_closure_sugar)]

fn main() {
    let f = |&: x: int| x;
    f.call((1i,));
}
