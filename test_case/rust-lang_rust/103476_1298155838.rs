rust
#![feature(let_chains)]
#![allow(irrefutable_let_patterns)]

fn it(_: &()) -> Box<dyn Tr<'_>> { todo!() }

trait Tr<'a> {}

fn f() {
    if let n = () && let _ = it(&n) {};
}
