rust
trait Service<R> {
    type Future;
}

struct Inner;

impl<R> Service<R> for Inner {
    type Future = ();
}

struct Outer<S> {
    inner: S,
}

impl<S> Service<()> for Outer<S>
where
    for<'a> S: Service<&'a ()>,
    for<'a> <S as Service<&'a ()>>::Future: Send,
{
    type Future = ();
}

fn is_service<S>(_: S)
where
    S: Service<()>
{}

fn main() {
    is_service(Outer { inner: Inner })
}
