rust
fn foo<'a: 'c, 'b: 'c, 'c>(...) -> impl Trait<'a, 'b> + 'c { .. }
