rust
impl<T> Box<T> {
    fn take(this: Self) -> (T, Box<MaybeUninit<T>>) { ... }
}
