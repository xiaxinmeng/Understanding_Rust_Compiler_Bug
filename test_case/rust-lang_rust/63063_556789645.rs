rust
type MyType<T, V> = impl Copy;
fn make_it<T>(val: T) -> MyType<T, T> { Ok(val) /* ERROR: we cannot see the underlying type here */ }
