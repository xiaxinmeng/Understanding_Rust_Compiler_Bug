 rust
use std::ops::*;

trait VectorSpace {}

trait AffineSpace where
    Self: Add<Self::Diff, Output = Self>,
    Self: Sub<Self, Output = Self::Diff>,
{
    type Diff: VectorSpace;
}
