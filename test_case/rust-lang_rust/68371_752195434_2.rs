rust
fn scanl<B, F, I>(f: F, mut b: B, iter: I) -> impl Iterator<Item = B>
where
    I: Iterator,
     // in Haskell everything is `Clone`, so this is equivalent
    B: Clone,
    // 1. may as well provide `FnMut` because we can
    // 2. `&mut B` is equivalent to `State B ()` in Haskell
    F: FnMut(I::Item, &mut B),
{
    iter.map(move |a| {
        f(a, &mut b);
        b.clone()
    })
}
