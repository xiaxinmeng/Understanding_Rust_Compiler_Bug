 rust
#![feature(macro_rules)]

macro_rules! foo {
    ($name:ident) => {
        /// testing
        pub struct $name;
    }
}

foo! { Foo }
foo! { Bar }

/// testing
foo! { Baz }

fn main() {
}
