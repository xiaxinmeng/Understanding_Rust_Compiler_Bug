rust
pub fn wrap_impl_trait<'c, 'i, I: 'i, O: 'i, F>(f: F) -> F
where
    F: Fn(&'i I) -> &'i O + 'c,
{
    f
}
