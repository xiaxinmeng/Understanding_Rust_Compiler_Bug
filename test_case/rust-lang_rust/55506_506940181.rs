rust
#[must_use]
trait Foo {}

impl Foo for () {}

fn foo() -> impl Foo {
    ()
}

fn bar() -> Box<dyn Foo> {
    Box::new(())
}


fn main() {
    foo();
    bar();
}
