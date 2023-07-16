rust
> impl/*<[i32; 1]>*/ NDArrayType<[i32; 1]> for [i32; 1] {}
> impl<T, U: NDArrayType<T>> NDArrayType<T> for [U; 1] {}
> 