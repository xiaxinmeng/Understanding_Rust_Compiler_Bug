rust
const fn convert_and_iterate<I: ~const IntoIterator<Item = i32>>(iter: I) -> i32 {
    iter.into_iter().next()
}
