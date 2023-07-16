rust
fn foo<'a, T, U>(x: &'a T, y: &'a U) where T: Trait<'a> { ... }
