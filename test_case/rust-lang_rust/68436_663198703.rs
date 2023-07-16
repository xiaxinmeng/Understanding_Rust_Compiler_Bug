rust
pub type TruncatedVector<T, const N: usize> where (N - 1) = Vector<T, { N - 1 }>;
