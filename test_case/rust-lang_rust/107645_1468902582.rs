rust
type Foo<T> = impl Trait;

fn foo<T>(x: T) -> Foo<T> { ..}
