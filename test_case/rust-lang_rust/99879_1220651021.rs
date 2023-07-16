
 617   │ impl<M, N> PartialDiv<N> for M
 618   │ where
 619   │     M: Integer + Div<N> + Rem<N, Output = Z0>,
 620   │ {
 621   │     type Output = Quot<M, N>;
 622   │     #[inline]
 623   │     fn partial_div(self, rhs: N) -> Self::Output {
 624   │         self / rhs
 625   │     }
 626   │ }
