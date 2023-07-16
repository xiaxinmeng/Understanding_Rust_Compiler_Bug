rust
pub trait Point: std::ops::Index<usize> where Self::Output: Float {};
