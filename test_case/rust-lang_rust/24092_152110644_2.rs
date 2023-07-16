 rust
use std::ops::*;

trait VectorSpace {}

trait AffineSpace where
    Self: Add<<Self as AffineSpace>::Diff, Output = Self>,
    Self: Sub<Self, Output = <Self as AffineSpace>::Diff>,
{
    type Diff: VectorSpace;
}
