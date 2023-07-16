rust
fn bar() {
    struct Foo;
    trait FooT {
        const FLAG: u32;
    }

    impl FooT for Foo {
        const FLAG: u32 = bogus.baz;
    }
}

