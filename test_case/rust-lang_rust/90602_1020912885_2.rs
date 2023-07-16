rust
#![feature(associated_type_bounds)]

const fn convert_and_iterate<I: ~const IntoIterator<IntoIter: ~const Iterator, Item = i32>>(iter: I) -> i32 {
    iter.into_iter().next()
}
