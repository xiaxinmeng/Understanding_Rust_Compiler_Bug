rust
#![feature(generic_associated_types)]

use std::ops::Deref;

trait SomeTrait {
    type Assoc<T = i32>: Deref<Target = T>;
}

fn main() {
}
