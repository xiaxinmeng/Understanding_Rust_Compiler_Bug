 rust
pub trait AddAssign<RHS> {
    #[inline]
    fn add_assign(&mut self, rhs: &RHS);
}
