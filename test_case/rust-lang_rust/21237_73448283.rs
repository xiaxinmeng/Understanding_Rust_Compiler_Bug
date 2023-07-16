 rust
#[derive(Debug)]
struct Foo<T>;

#[derive(Debug)]
struct Bar<T>(Foo<T>);

trait Trait { type Type; }

#[derive(Debug)]
struct Baz<T: Trait>(T::Type);
