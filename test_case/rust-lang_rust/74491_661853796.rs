rust
fn test<T>() {
    let _ = [0; SOME_MONOMORPHIC_CONST * std::mem::size_of::<T>()];
}
