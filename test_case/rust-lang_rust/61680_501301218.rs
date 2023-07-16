rust
unsafe trait HasSameMemoryLayoutAs<T> {}

impl Vec<T> {
    fn flatten_in_place<U>(self) -> Vec<U>
    where
        U: HasSameMemoryLayoutAs<T>,
    {
        // Handwaving goes here
    }
}
