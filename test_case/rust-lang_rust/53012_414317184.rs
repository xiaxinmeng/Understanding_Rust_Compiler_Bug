rust
pub struct Foo {
    // do not allow
    nop! {
        pub foo_field: u32,
    }
}
