rust
trait Future {
    type Item;
}

struct PollFn<F> {
    _i: F,
}
fn poll_fn<T, F>(_: F) -> PollFn<F>
where
    F: FnMut() -> T,
{
    unimplemented!()
}
impl<T, F> Future for PollFn<F>
where
    F: FnMut() -> T,
{
    type Item = T;
}

struct JoinAll<I>
where
    I: IntoIterator,
    I::Item: Future,
{
    _f: I::Item,
}
fn join_all<I>(_: I) -> JoinAll<I>
where
    I: IntoIterator,
    I::Item: Future,
{
    unimplemented!()
}
impl<I> Future for JoinAll<I>
where
    I: IntoIterator,
    I::Item: Future,
{
    type Item = I;
}

fn f<'a, T: 'a>(r: &'a T) -> impl Future + 'a {
    let requests = Vec::<()>::new();
    join_all(requests.into_iter().map(move |_| {
        poll_fn(move || {
            r;
        })
    }))
}

fn main() {
    || -> Box<Send> { Box::new(f(&())) };
}
