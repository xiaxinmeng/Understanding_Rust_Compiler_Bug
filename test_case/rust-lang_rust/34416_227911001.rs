 rust
pub trait Future: 'static {
    fn forget(self) where Self: Sized {
        Box::new(Map(self)) as Box<Future>;
    }
}

pub struct Map<A>(A);
impl<A: Future> Future for Map<A> {}

pub struct Promise;
impl Future for Promise {}

fn main() {
    Promise.forget();
}
