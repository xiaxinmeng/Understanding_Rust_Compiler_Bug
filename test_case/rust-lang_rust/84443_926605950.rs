rust
use std::collections::{HashMap, HashSet};

pub struct A {
    items: HashSet<u32>,
}
pub struct B {
    a: A,
    map: HashMap<u32, u32>,
}
impl B {
    fn new() -> Self { todo!() }
}

fn main() {
    B::new().items.contains("");
}
