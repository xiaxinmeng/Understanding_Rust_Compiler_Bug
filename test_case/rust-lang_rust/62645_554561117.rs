rust
const fn batch_size<T: Sized>() -> usize {
    32 / std::mem::size_of::<T>()
}

fn x<T: Sized>() {
    [1; batch_size::<T>()];
}
