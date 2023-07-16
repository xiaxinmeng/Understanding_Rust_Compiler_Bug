rust
impl<S, R> hyper::service::Service<R> for AdaptorService<S>
where
    S: Service<R>,
{
    type Response = S::Response;

    type Error = io::Error;

    type Future = AdaptorFuture<S::Future>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: R) -> Self::Future {
        AdaptorFuture {
            inner: self.inner.call(req),
        }
    }
}
