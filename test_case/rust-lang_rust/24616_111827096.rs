 rust
impl<Lhs: Peano + AddPeano<Rhs>, Rhs: Peano> AddPeano<Rhs> for Succ<Lhs> {
    type Output = Succ<<Lhs as AddPeano<Rhs>>::Output>;
}
