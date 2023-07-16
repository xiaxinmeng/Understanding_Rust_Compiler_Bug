rust
#![feature(generic_associated_types)]

use std::iter::Map;
use std::slice::Iter;

pub trait WithStrings {
    type Strings<'a>: Iterator<Item = String>
    where
        Self: 'a;

    fn with_strings<R>(self, f: impl FnOnce(Self::Strings<'_>) -> R) -> R;
}

impl<T: ToString, const N: usize> WithStrings for [T; N] {
    type Strings<'a>
    where
        Self: 'a,
    = Map<Iter<'a, T>, fn(&T) -> String>;

    fn with_strings<R>(self, f: impl FnOnce(Self::Strings<'_>) -> R) -> R {
        f(self.iter().map(T::to_string))
    }
}
