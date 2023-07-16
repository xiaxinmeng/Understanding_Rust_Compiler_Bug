rust
impl<'a, Req> Service<&'a Req> for u8 {
    type Output= &'a Req;
    fn call(req: &'a Req) -> Self::Output {
        req
    }
}
