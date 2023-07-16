 rust
pub trait Chain: Handler {
    fn new<H: Handler>(H) -> Self;
    fn link<B, A>(&mut self, (B, A)) where A: AfterMiddleware, B: BeforeMiddleware;
    fn link_before<B>(&mut self, B) where B: BeforeMiddleware;
    fn link_after<A>(&mut self, A) where A: AfterMiddleware;
    fn around<A>(&mut self, A) where A: AroundMiddleware;
}
