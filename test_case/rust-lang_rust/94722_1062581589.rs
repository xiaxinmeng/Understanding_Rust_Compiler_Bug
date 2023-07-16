rust
impl<X> NDArrayType<X> for X {}
impl<T, U: NDArrayType<T>> NDArrayType<T> for [U; 1] {}
