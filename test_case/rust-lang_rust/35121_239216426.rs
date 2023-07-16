
trait Baz { ... }

trait Foo {
    type Bar: Baz;

    fn do_something(&self) -> Self::Bar;
}
