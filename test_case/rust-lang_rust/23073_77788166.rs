 rust
trait What { type T; }
trait Bar {
    type Foo;
    type FooT = Self::Foo::T;
}
