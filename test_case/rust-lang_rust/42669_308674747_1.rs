rust
trait Foo {
    fn foo<'a>(x: &'a i32, y: &'a i32) -> &'a i32;
}

impl Foo for () {
    fn foo<'a>(x: &i32, y: &'a i32) -> &'a i32 {
        // adding a `'a` to `x` would be legal here...
        y
    }
}
