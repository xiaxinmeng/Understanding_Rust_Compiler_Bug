rust
trait Foo {
    fn bar(&self) -> Self where Self: Sized;
}

impl Foo for str {
    fn bar(&self) -> Self where for<'a> str: Sized { unimplemented!() }
}
