rust
fn test<T>() {
    let _ = [0; std::mem::size_of::<T>()];
}
