rust
pub struct Foo {
    // If this expands to more than one field, I can no longer write
    // the struct literal `Foo { foo_field: 0 }`.
    not_noop! {
        pub foo_field: u32,
    }
}

impl Foo {
    // If this expands to more than one impl item there is no disruption.
    not_noop! {
        fn method(&self) {}
    }
}
