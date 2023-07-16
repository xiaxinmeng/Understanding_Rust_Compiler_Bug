rust
mod M {
    pub type MyType<T, E> = impl Copy;
}
fn make_it<T: Copy>(val: T) -> M::MyType<T, T> { Result::<T, T>::Ok(val) }
