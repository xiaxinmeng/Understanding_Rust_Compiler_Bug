rust
fn foo<T>(arg: impl Trait) {}

foo::<u8, _>(); // the argument for `arg` is inferred, not specified
