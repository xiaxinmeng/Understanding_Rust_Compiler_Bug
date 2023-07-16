rust
#![crate_name = "foo"]

pub struct Foo;

impl Foo {
    // @has 'foo/struct.Foo.html' '//code' 'pub fn f()'
    pub fn f() {}
}
