 rust
trait Foo { type FooT; }
trait Bar { }
trait FooBar: Foo<FooT=usize> + Bar { }
type BoxedFooBar = Box<FooBar>;
