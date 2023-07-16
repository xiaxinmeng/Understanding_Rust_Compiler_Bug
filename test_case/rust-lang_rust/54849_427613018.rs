rust
pub struct A<T: Sized> {
    data: [T; A::<T>::SIZE],
}

impl<T: Sized> A<T> {
    const SIZE: usize = 4;
}
