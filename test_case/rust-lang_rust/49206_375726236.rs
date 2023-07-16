rust
#![feature(optin_builtin_traits)]

#[derive(Debug)]
struct Foo {
    value: u32,
}

impl !std::marker::Sync for Foo {}

const F: &'static Foo = &Foo { value: 1 };

fn main() {
}
