rust
#![feature(existential_type)]

trait Future {}
impl Future for () {}

existential type CacheFuture: Future + 'static;

fn cache<I>(_: I) -> CacheFuture
where
    I: IntoIterator<Item = ()>,
{
    ()
}

pub fn cache_slice<'a>(glyphs: &[()]) -> impl Future + 'static {
    cache(glyphs.iter().cloned())
}
