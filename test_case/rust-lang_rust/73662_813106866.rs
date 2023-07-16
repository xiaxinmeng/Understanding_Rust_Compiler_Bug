Rust
trait Enum {
    type Repr;
    /// Returns the maximum possible index into an array containing every variant of this enum.
    /// Returns `None` if the enum has no variants.
    const fn max_variant() -> Option<Repr>;
}
