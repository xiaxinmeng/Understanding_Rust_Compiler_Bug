 rust
impl Foo {
    fn bar(&self) {
        struct Inner;
        impl Inner {
            fn baz(&self) {}
        }
    }
}
