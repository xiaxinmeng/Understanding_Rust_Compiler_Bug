rust
fn generic<T>() {
    let a = Vec::<i32>::new(); // The type `Vec<i32>` is "global".
    let a = Vec::<T>::new(); // The type `Vec<T>` is "local", because it depends on `T`.
}
