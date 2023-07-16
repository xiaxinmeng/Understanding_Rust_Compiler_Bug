rust
> impl/*<[i32; 1]>*/ NDArrayType<[i32; 1]> for [i32; 1] {}
> impl<[i32; 1], i32: NDArrayType<[i32; 1]>> NDArrayType<[i32; 1]> for [i32; 1] {}
> 