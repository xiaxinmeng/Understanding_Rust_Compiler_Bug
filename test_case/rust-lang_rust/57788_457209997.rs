rust
pub fn wrap_impl_trait<'c, I, O>(
    f: impl for<'a> Fn(&'a I) -> &'a O + 'c,
) -> impl for<'a> Fn(&'a I) -> &'a O + 'c
{
    f
}
