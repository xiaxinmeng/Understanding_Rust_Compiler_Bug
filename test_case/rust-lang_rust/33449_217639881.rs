 Rust
#![allow(unused)]

fn f<F>() -> Result<Option<F>, ()> {
    panic!()
}

fn g<G>() -> Option<G> where Option<G>: Sized {
    f::<Option<G>>().unwrap().unwrap()
}

fn main() {}
