 rust
pub trait BitCount {
    fn population_count(&self) -> Self;
    fn leading_zeros(&self) -> Self;
    fn trailing_zeros(&self) -> Self;
}
