Rust
struct Foo;

mod x {
    impl ::Foo { pub fn foo() {} }
}

mod y {
    macro m($foo:expr) { $foo.foo(); }
    fn example(foo: ::Foo) { m!(foo) }
}
