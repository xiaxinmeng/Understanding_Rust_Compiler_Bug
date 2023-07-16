rust
enum Maybe<T>{
    Just(T),
    Nothing,
}

impl<T> Maybe<T>{
    pub const INITIAL_VALUE: Self = Self::Nothing;

    pub fn is_initial(&self) -> bool {
        if let Self::INITIAL_VALUE = *self {
            true
        } else {
            false
        }
    }
}
