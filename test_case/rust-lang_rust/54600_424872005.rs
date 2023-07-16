rust
#![feature(nll)]
#![feature(impl_trait_in_bindings)]

use std::fmt::Debug;

fn foo() -> String {
    let x: impl Debug = 22;
    format!("{}", x)
}

fn main() { }
