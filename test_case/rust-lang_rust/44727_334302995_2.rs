rust
abstract type Foo<'a, 'b, T, 'c>: impl Iterator<Item = &'c T>;
fn foo<'a,'b,T>(...) -> Foo<'static, 'static, T, 'a>
