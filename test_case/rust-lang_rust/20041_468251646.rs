rust
trait Trait {
    type Foo;
    type Bar;
}

struct Struct<T>(T) where T: Trait, T::Foo != T::Bar;
