
impl<'a, C> Mul<C> for &'a SymMonomial where C: Mul<i64, Output=i64>{
    // type Output = SymMonomial;
    fn mul(self, rhs: C) -> Self::SymMonomial {
        SymMonomial{coefficient: rhs * self.coefficient, powers: self.powers.clone()}
    }
}
