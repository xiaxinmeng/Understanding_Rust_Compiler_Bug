rust
use std::ops::Mul;
pub trait Diff {
    type ValueType;
    type ForwardDiff;
}
pub struct Multiplication<L>
where
    L: Diff,
{
    left: L,
}
impl<L> Diff for Multiplication<L>
where
    L: Diff,
    L::ForwardDiff: Diff,
    Multiplication<L::ForwardDiff>: Diff<ValueType = ()>,
{
    type ValueType = ();
    type ForwardDiff = ();
}
impl<L: Diff> Mul for Multiplication<L> {
    type Output = Multiplication<Self>;
}
