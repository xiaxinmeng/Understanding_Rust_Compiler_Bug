rust
// foo.rs
#![crate_type = "lib"]
#![feature(conservative_impl_trait)]

fn bar() {}

pub fn foo() -> impl FnMut() {
    || {
        bar()
    }
}

// bar.rs
extern crate foo;

fn main() {
    foo::foo()();
}
