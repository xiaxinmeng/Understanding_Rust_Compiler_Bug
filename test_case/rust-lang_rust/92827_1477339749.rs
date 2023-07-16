rust
pub trait TensorShape1D {
    const SHAPE: usize;  // i.e. length
}

pub fn add<const L: usize, LEFT_TENSOR: TensorShape1D<SHAPE=L>, RIGHT_TENSOR: TensorShape1D<SHAPE=L>>(left: LEFT_TENSOR, right: RIGHT_TENSOR) {
    todo!()
}
