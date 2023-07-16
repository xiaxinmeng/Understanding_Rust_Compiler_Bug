rust
#![allow(incomplete_features)]
#![feature(const_fn, const_trait_impl)]

use std::ops::Deref;

struct Strange {}

impl const Deref for Strange {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        "What a weird way to use deref!"
    }
}

const STR: &str = &*Strange {};

fn main() {
    println!("{}", STR);
}
