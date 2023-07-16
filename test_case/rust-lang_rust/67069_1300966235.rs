rust
struct Foo<T> { x: T }

// error:
impl<String> Foo<String> {}
// should be:
impl Foo<String> {}
