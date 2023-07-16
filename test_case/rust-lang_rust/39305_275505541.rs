
fn foo(x: StaticStr) -> &str { x }
fn bar<'a, T: WithLifetime<'a>>(_: T::Output) -> &str { "baz" }
