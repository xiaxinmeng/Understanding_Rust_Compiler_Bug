rust
#![feature(matches_macro)]

enum Maybe<T>{
    Just(T),
    Nothing,
}

impl<T> Maybe<T>{
    pub const INITIAL_VALUE: Self = Self::Nothing;

    pub fn is_initial(&self) -> bool {
        matches!(*self, Self::INITIAL_VALUE)
    }
}
