rust
trait Foo {
    type Assoc: Bar;
}

trait Bar {}

fn test<T: Foo<Assoc = S>, S>() {}
