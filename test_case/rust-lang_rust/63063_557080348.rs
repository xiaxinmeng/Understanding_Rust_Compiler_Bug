rust
type MyType<T, E> = impl Copy;
fn make_it<T: Copy>(val: T) -> MyType<T, T> { Result::<T, T>::Ok(val) }
