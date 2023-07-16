rust
type Foo<T: Debug> = (impl Foo<T>, T);
