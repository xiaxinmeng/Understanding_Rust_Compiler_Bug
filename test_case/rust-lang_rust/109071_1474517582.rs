rust
#![allow(incomplete_features)]
#![feature(inherent_associated_types)]

struct Windows<T> {}

impl<T> Windows<()> {
    type Item = T;

    fn next() -> Self::Item {}
}

impl<T> Windows<T> {
    fn gates() -> Self::Item {}
}
