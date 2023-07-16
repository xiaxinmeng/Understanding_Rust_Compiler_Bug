rust
type Foo = _ as impl Future;

fn foo() -> Foo { async { ... } }
