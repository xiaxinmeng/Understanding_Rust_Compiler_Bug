 rust
fn test<F: for<'x> Lambda<&'x str>>(_: F) {}

trait Lambda<T> {
    type Output;
}

impl<T, U> Lambda<T> for fn(T) -> U {
    type Output = U;
}

struct Compose<F,G>(F,G);
impl<T,F,G> Lambda<T> for Compose<F,G>
where F: Lambda<T>, G: Lambda<F::Output> {
    type Output = G::Output;
}

fn bad<T>(f: fn(&'static str) -> T) {
    test(Compose(f, std::mem::drop as fn(T)));
}
