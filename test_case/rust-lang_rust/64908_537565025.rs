rust
pub trait ArrowPrimitiveType {
    type Native;
}

pub fn new<T: ArrowPrimitiveType>() {
    assert_eq!(0, std::mem::size_of::<T::Native>());
}
