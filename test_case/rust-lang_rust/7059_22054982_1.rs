 rust
impl<LHS: Add<RHS, LHS>, RHS> AddAssign<RHS> for LHS {
    fn add_assign(&mut self, rhs: &RHS) {
        *self = *self + *rhs;
    }
}
