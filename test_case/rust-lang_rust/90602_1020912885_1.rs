rust
const fn convert_and_iterate<I: ~const IntoIterator<IntoIter = J, Item = i32>, J: ~const Iterator<Item = i32>>(iter: I) -> i32 {
    iter.into_iter().next()
}
