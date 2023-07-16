rust
use std::future::Future;

struct Response<'r>(&'r str);
struct Request<'i>(&'i str);

trait Handler<'r> {
    type Output: Future<Output = Response<'r>>;
    fn call(&self, request: &'r Request<'_>) -> Self::Output;
}

impl<'r, F, Fut> Handler<'r> for F
    where F: Fn(&'r Request<'_>) -> Fut, Fut: Future<Output = Response<'r>>
{
    type Output = Fut;

    fn call(&self, request: &'r Request<'_>) -> Self::Output {
        self(request)
    }
}

fn check<H: for<'r> Handler<'r>>(_handler: H) { }

fn main() {

    fn handler<'r>(request: &'r Request<'_>) -> impl Future<Output = Response<'r>> {
        async move {
            Response(&request.0)
        }
    }

    async fn _handler3<'r>(request: &'r Request<'_>) -> Response<'r> {
        Response(&request.0)
    }

    check(handler);
    check(_handler3); // FIXME: Why doesn't this work? What lifetimes does the generated `Future` have?
}
