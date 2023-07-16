 rust
    #[deprecated="renamed to `into_vec`"]
    fn to_vec(self) -> Vec<T> { self.into_vec() }

    #[deprecated="renamed to `into_sorted_vec`"]
    fn to_sorted_vec(self) -> Vec<T> { self.into_sorted_vec() }

    fn into_vec(self) { /* current code in `.to_vec` */ }
    fn into_sorted_vec(self) { /* current code in `.to_sorted_vec` */ }
