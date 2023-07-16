rust
pub fn foo<T>(lists: &[&[T]]) {
    let (first, rest) = lists.split_first().unwrap_or((&(&[][..]), &[]));
}
