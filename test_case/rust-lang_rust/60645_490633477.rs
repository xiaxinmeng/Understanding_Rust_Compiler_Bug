rust
impl<P> Future for P
where
    P: Unpin + ops::DerefMut,
    P::Target: Future + Unpin,
{ ... }
