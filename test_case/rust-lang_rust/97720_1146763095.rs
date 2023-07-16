rust
trait Foo<'a> {}
fn foo(t: impl Foo<'_>) {} //~ ERROR missing lifetime specifier
async fn async_foo(t: impl Foo<'_>) {} //~ OK (since 1.39.0)
