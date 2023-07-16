 rust
impl<Lhs: Peano + Add<Rhs>, Rhs: Peano> Add<Rhs> for Succ<Lhs> where <Lhs as Add<Rhs>>::Output: Peano {
    type Output = Succ<<Lhs as Add<Rhs>>::Output>;
    fn add(self, rhs: Rhs) -> Self::Output { unreachable!() }
}
