rust
fn scanl<B, F, I>(f: F, mut b: B, iter: I) -> impl Iterator<Item = C>
where
    I: Iterator,
    F: FnMut(I::Item, &mut B) -> C,
{
    iter.map(move |a| f(a, &mut b))
}
