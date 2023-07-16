rust
impl<T, const N: usize> Vector<T, { N }> 
where
     (N - 1)
{
    pub fn truncate(self) -> (TruncatedVector<T, { N }>, T) {
        ...
    }
}
