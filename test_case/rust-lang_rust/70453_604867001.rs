rust
fn f<T>() {
    const _: usize = std::mem::size_of::<T>(); // ERROR can't use generic parameters from outer function
}
