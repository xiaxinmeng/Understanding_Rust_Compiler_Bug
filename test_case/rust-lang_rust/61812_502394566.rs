rust
pub trait Rem<Rhs=Self> {
    type Output = Self;
    fn rem(self, rhs: Rhs) -> Self::Output;
}
