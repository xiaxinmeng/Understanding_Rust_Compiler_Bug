rust
impl<P, Q, O> Sub<Combinator<Q>> for Combinator<P>
{
  type Output = Combinator<Left<P,Q,O>>;

  fn sub(self, other: Combinator<Q>) -> Self::Output {
    Combinator(Left(self.0, other.0, PhantomData))
  }
}
