rust
pub struct Pending<T> {
    _data: marker::PhantomData<T>,
}
