rust
#![feature(nll)]

fn test<'a>() {
    let _: for<'b> fn(&'b ()) = |_:&'a ()| {};
}

fn main() {
    test();
}
