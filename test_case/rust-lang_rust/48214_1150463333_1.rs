rust
    // We can never call `foo`, but we can `impl Foo for str`!
    fn foo() -> str where str: Sized {
        unreachable!()
    }
