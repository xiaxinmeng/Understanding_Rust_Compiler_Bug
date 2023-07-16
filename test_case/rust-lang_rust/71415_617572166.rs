rust
#![feature(const_generics)]
#![feature(structural_match)]

use std::marker::{StructuralPartialEq, StructuralEq};

struct Container<T: StructuralPartialEq + StructuralEq + PartialEq + Eq, const V: T>;

fn main() {
    let container1: Container<i32, {42}> = Container;
    let container2: Container<&'static str, "Hello world"> = Container;
}
