 rust
trait FooBar: Foo + Bar {}
impl<T: Foo + Bar> FooBar for T {}
