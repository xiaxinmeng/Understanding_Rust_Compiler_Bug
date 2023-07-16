rust
fn successors<T, F>(first: Option<T>, mut succ: F) -> impl Iterator<Item = T>
where
    F: FnMut(&T) -> Option<T>,
{
    std::iter::unfold(first, move |next| {
        next.take().map(|item| {
            *next = succ(&item);
            item
        })
    })
}