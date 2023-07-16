rust
#![feature(negative_impls)]

trait Bar {
    fn bar(&self);
}

impl<T: Copy> !Bar for T {}

fn foo(x: ()) {
    x.bar()
}

fn main() {}
