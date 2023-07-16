 rust
trait Sum: Add<Rhs=Self> {
    /// Sum of an empty sequence, i.e. "zero".
    fn neutral_element() -> Self;
}
trait Product: Mul<Rhs=Self> {
    /// Product of an empty sequence, i.e. "one".
    fn neutral_element() -> Self;
}
