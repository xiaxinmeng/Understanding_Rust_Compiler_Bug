rust
pub enum ArrayFillError {
    /// It might be worth it to return an `[MaybeUnit<T>], usize` instead
    TooFew(usize),
    /// there were too many elements, with `T` being the `n+1`th element
    TooMany(T)
}
