
type Foo = ...
trait Bar {
    type Foo;
    fn foo() -> Foo;   // <- which Foo does this refer to?
}
