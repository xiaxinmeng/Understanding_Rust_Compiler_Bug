rust
impl<U, V> AsRef<V> for &'_ U
where
    U: ?Sized + AsRef<V>,
    V: ?Sized;
