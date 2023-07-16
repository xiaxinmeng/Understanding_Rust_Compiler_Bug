rust
struct Map<A, F> where A: Future {
    a: A,
    f: F,
}

trait Future {
    type Item;
    type Error;

    fn tailcall(&mut self) -> Option<Box<Future<Item=Self::Item, Error=Self::Error>>>;
}

impl<A, F, U> Future for Map<A, F>
where
    A: Future,
    F: FnOnce(A::Item) -> U + Send + 'static,
    U: Send + 'static,
{
    type Item = U;
    type Error = A::Error;

    fn tailcall(&mut self)
                -> Option<Box<Future<Item=Self::Item, Error=Self::Error>>> {
        None
    }
}

fn main() {}
