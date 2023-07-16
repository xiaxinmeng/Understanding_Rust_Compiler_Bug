rust
#![feature(existential_type)]

use std::fmt::Debug;

macro_rules! this_is_an_item {
    ($i:item) => { $i };
}

fn main() {
    this_is_an_item! {
        existential type Existential: Debug;
    }

    fn f() -> Existential {}
    println!("{:?}", f());
}
