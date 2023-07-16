rust
impl<'a, Req> Service<'a, Req> for u8 {
    type Future = impl Future;
    fn call(req: &'a Req) -> Self::Future where Req: 'a {
        async move { let x = req; }
    }
}
