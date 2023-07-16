rust
trait Foo {
    fn foo() -> Self where Self: Sized;
    fn bar(&self);
}

impl Foo for str {
    // We can't compile without defining *something* for foo, even though it could never be called!
    fn bar(&self) {
        println!("bar");
    }
}
