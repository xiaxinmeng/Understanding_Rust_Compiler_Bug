rust
trait Service<Req> {
    type Output;
    fn call(req: Req) -> Self::Output;
}

impl<'a, Req> Service<&'a Req> for u8 {
    type Output= impl Copy;
    fn call(req: &'a Req) -> Self::Output {
        req
    }
}
