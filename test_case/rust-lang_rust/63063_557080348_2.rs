rust
type MyType<T> = impl Copy;
fn make_it<T: Copy>(val: T) -> MyType<T> { Result::<T, T>::Ok(val) }
