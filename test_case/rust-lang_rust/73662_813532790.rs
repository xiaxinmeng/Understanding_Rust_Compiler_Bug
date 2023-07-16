rust
/// Returns the maximum possible index into an array containing every variant of the enum `T`.
/// Returns `None` if the enum has no variants.
fn max_variant<T>() -> Option<<T as DiscriminantKind>::Discriminant>
