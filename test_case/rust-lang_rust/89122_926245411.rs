rust
type Thing<T> where T: IntoIterator = T::IntoIter
where
    T::Item: Clone;
