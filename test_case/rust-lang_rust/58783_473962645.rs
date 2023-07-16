rust
    fn clone(&self) -> Vec<T> {
        <[T]>::to_vec(&**self)
    }
