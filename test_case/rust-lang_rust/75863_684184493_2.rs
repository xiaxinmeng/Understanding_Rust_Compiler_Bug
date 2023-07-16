rust
pub trait Accum<Rhs = Self> {
    type Output;
    fn concat(self, rhs: Rhs) -> Self::Output;
}
