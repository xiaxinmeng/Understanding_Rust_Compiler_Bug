Rust
trait Foo<'a> {
    fn foo<'b>(x: &'a &'b u32) {}
}

impl<'a> Foo<'a> for () {
    fn foo(x: &'a &'a u32) {} //~ ERROR
}

