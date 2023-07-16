 rust
struct MatrixPtr_<Sized? T> {
    stride: uint,
    data: T,
}
pub type MatrixPtr = MatrixPtr_<[T]>;
