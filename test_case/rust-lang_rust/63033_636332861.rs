rust
pub struct Request<'a> {
    ...
}

impl<'a> Request<'a> {
    async fn send(self) -> Result<(), Error> {
        ...
    }
}

// does not compile
// "cannot infer an appropriate lifetime for lifetime parameter `'a` due to conflicting requirements"
impl<'a> IntoFuture for Request<'a> {
    type Output = Result<(), Error>;
    type Future = impl Future<Output = Self::Output>;

    fn into_future(self) -> Self::Future {
        self.send()
    }
}

// does not compile
// "hidden type for `impl Trait` captures lifetime that does not appear in bounds"
impl<'a> IntoFuture for Request<'a> {
    type Output = Result<(), Error>;
    type Future = impl Future<Output = Self::Output> + 'a;

    fn into_future(self) -> Self::Future {
        self.send()
    }
}
