rust
pub struct AdaptorFuture<F> {
    inner: F,
}

impl<F, T> Future for AdaptorFuture<F>
where
    F: Future<Output = T>,
{
    type Output = Result<T, io::Error>;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        panic!()
    }
}
