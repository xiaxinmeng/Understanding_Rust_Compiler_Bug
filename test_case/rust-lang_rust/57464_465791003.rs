rust
pub struct AndThen<B, F>(B, F);
fn and_then<F, B>(_: F) -> AndThen<B, F> where F: FnOnce() -> B { unimplemented!() }

pub trait Trait { }
impl<B, F> Trait for AndThen<B, F> { }

pub struct JoinAll<I> where I: Iterator { _elem: std::marker::PhantomData<I::Item> }
pub fn join_all<I>(_i: I) -> JoinAll<I> where I: Iterator { unimplemented!() }

pub struct PollFn<F, T>(F, std::marker::PhantomData<fn () -> T>);
pub fn poll_fn<T, F>(_f: F) -> PollFn<F, T> where F: FnMut() -> T { unimplemented!() }

impl<B, I: Iterator, F> Iterator for Map<I, F> where F: FnMut(I::Item) -> B {
    type Item = B;
    fn next(&mut self) -> Option<B> { unimplemented!() }
}

struct Map<I, F> { iter: I, f: F }

fn main() { let _b: Box<Trait + Send> = Box::new(graphql()); }

fn graphql() -> impl Trait
{
    let local = ();
    let m = |_: ()| poll_fn(|| { local; });
    let v = Map { iter: std::iter::once(()), f: m };
    let f = || join_all(v);
    and_then(f)
}
