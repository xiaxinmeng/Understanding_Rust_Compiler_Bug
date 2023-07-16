rust
fn bind_service<S>()
where
    S: for<'a> Service<&'a ()>,
{
    serve::<_, Wrapper<S>>();
}

fn serve<F, S: for<'a> Service<&'a (), Future = F>>() {}

trait Service<Request> {
    type Future;
}

struct Wrapper<S>(S);

impl<'a, S> Service<&'a ()> for Wrapper<S>
where
    S: for<'b> Service<&'b ()>,
{
    type Future = <S as Service<&'a ()>>::Future;
}
