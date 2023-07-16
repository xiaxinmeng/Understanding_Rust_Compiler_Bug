rust
fn run() {
    struct Foo;
    impl Foo {
        const A: () = assert!(false);
    }
    Foo::A
}
