
trait Future {}
impl Future for () {}

fn cache<I>(_: I) -> impl Future + 'static
where
    I: IntoIterator<Item = ()>,
{
    ()
}

fn cache_slice<'a>(glyphs: &[()]) -> impl Future + 'static {
    cache(glyphs.iter().cloned())
}
