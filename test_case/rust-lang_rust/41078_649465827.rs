rust
pub struct Request<'a, 'b: 'a> {
    a: &'a (),
    b: &'b (),
}
pub trait Handler: Send + Sync + 'static {
    fn handle(&self, a: &mut Request) -> Result<(),()>;
}
impl<F> Handler for F where F: Send + Sync + 'static + Fn(&mut Request) -> Result<(),()> {
    fn handle(&self, _: &mut Request) -> Result<(),()> {
        unimplemented!()
    }
}
fn make_handler(h: &'static dyn Handler) -> Box<dyn Handler> {
    Box::new(move |req| h.handle(req))
}
