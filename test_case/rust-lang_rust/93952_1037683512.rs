rust
struct Foo;

impl Foo {
    fn foo() {
        impl Self { fn bar() {} }
    }
}
