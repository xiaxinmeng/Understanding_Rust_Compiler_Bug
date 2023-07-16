rust
impl<'r, S, State, Res, Err> Service<WebRequest<'r, State>> for MiddlewareService<S>
where
    S: for<'r2> Service<WebRequest<'r2, State>, Response = Res>,
{
    type Response = Res;
    type Future<'f> = impl Future<Output = Res> + Captures<'r> + Captures<'f> where Self: 'f;

    fn call(&self, mut req: WebRequest<'r, State>) -> Self::Future<'_> {
        async move { self.0.call(req).await }
    }
}
