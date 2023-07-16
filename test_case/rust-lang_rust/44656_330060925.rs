rust
pub trait Point
  : std::ops::Index<usize, Output = <Self as Point>::Output> {
     type Output: Float;
}
