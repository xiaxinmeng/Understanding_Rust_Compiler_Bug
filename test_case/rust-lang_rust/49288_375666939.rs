rust
trait SendFoo = Foo<FooFut = impl Send, BarFut = impl Send, BazFut = impl Send>;

fn foo() -> impl SendFoo { ... }
