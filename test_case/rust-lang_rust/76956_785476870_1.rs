rust
fn foo<'a>(x: impl FnOnce(<Container<'a> as Trait>::Assoc)) {
