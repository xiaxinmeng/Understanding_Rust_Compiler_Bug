rust
fn with_positive<'a, F>(vals: &'a [i32], fun: F)
where
    F: Fn(FilteredIter<'a>),
{
    fun(filter_positive(vals));
}
