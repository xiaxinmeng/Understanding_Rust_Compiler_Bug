rust
trait Foo {
    fn foo<'a: 'a>(); // early
}

impl Foo for () {
    fn foo<'a>() {} // late, but should be early
}
