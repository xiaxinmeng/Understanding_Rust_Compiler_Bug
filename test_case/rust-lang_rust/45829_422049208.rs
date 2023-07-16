rust
#![allow(unused_imports)]

mod foo {
    pub struct A;
    pub struct B;
}

use foo::{A, B as A};

fn main() {}
