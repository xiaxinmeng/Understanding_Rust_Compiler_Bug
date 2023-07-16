rust
trait VectorSpace
where
    Self: ...,
    Self: Into<[Self::Scalar, Self::D]> + From<[Self::Scalar, Self::D]>,
{
    type Scalar;
    const D: usize;
}
