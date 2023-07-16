rust
pub fn extend<'c, 'i, I: 'i, M: 'i, O: 'i>(
    inner: impl Fn(&'i I) -> &'i M + 'c,
    outer: impl Fn(&'i M) -> &'i O + 'c,
) -> impl Fn(&'i I) -> &'i O + 'c {
    move |i| outer(inner(i))
}
