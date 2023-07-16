rust
impl<[A; 1]> NDArrayType<[A; 1]> for [A; 1] {}
impl<[A; 1], A: NDArrayType<[A; 1]>> NDArrayType<[A; 1]> for [A; 1] {}
