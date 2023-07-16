rust
trait ExtraFoo<trait X> = Foo<FooFut = impl X, BarFut = impl X, BazFut = impl X>;

fn foo() -> impl ExtraFoo<Send> { ... }
