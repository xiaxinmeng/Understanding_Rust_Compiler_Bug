rust
fn want_static<T: 'static>() {}

fn test<Iter, X>()
where
    Iter: Iterator<Item = X> + 'static,
{
    want_static::<Iter::Item>();
    //~^ ERROR the parameter type `X` may not live long enough
}
