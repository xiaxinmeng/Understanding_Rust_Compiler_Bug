 rust
pub trait Add<RHS = Self> where <Self as Add<RHS>>::Output: Sized {
    type Output;
    fn add(self, rhs: RHS) -> <Self as Add<RHS>>::Output;
}
