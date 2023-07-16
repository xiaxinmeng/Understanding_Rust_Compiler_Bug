rust
abstract type Foo<'a>: Trait<'a>;
fn foo(x: &u32) -> Foo<'_> { ... }
