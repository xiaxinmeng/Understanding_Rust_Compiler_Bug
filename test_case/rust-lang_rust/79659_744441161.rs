rust
fn next_n<const N: usize>(&mut self) -> Result<[T; N], ArrayVec<T, N - 1>>;
