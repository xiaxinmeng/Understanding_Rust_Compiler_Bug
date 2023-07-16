rust
trait Future {
    type Output;
}
impl<T> Future for T {
    type Output = T;
}

trait Service<Request> {
    type Error;
    type Future: Future<Output = Self::Error>;
    fn fail(&self, r: Request) -> Self::Future;
}

struct Foo;
struct A;

impl Service<A> for Foo {
    type Error = ();
    type Future = Self::Error;
    fn fail(&self, r: A) -> Self::Future {
        ()
    }
}

impl Foo {
    fn fail_fast(&self) -> impl Future<Output = <Self as Service<A>>::Error> {
        self.fail(A)
    }
}
