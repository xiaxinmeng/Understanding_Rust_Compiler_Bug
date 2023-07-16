rust
// err: concrete type differs
fn bar() -> impl Iterator<Item = (TAIT, impl Trait)> {
    std::iter::once((foo(), Bar))
}
