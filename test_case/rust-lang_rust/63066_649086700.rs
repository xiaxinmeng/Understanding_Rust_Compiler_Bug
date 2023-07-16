rust
trait SomeTrait {
    type Foo: Bar;

    fn foo() -> Self::Foo;
}

impl SomeTrait for X {
    type Foo = impl Bar;

    fn foo() -> Self::Foo { ... }
}
