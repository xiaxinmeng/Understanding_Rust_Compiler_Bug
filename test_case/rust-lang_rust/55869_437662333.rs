rust
fn successors<T, F>(mut next: Option<T>, mut succ: F) -> impl Iterator<Item = T>
where
    F: FnMut(&T) -> Option<T>,
{
    std::iter::iterate(move || {
        next.take().map(|item| {
            next = succ(&item);
            item
        })
    })
}
