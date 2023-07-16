rust
fn all_explicits_are_early<'a, 'b>(arg: &Foo<'a, 'b>) -> Bar<'a, 'b> where 'a: 'a, 'b: 'b { ... }
