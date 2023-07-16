rust
trait Foo {
    fn foo() {}
}

trait Bar: Sized {}

impl Foo for dyn Bar {}

fn static_poly<T: Foo>() {
    T::foo();
}

fn main() {
    Bar::foo(); // Works now!
    static_poly::<dyn Bar>(); // Does not work currently
}
