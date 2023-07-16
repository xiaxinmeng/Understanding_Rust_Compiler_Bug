rust
fn test0(foo: impl Trait) -> <impl Trait as std::ops::Sub>::Output {
    foo - foo
}
