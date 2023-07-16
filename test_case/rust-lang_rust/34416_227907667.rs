 rust
pub trait Future: 'static {
    type Item;
    type Error;

    fn boxed(self) -> Box<Future<Item=Self::Item, Error=Self::Error>>
        where Self: Sized
    {
        Box::new(self)
    }

    fn map(self) -> Map<Self>
        where Self: Sized,
    {
        loop {}
    }

    fn forget(self) where Self: Sized {
        self.map().boxed();
    }
}

// =============================================================================

pub struct Map<A> {
    _future: A,
}

impl<A> Future for Map<A>
    where A: Future,
{
    type Item = ();
    type Error = A::Error;
}

// =============================================================================

pub struct Promise;

impl Future for Promise {
    type Item = ();
    type Error = ();
}

// =============================================================================

fn foo() -> Promise {
    loop {}
}

fn main() {
    foo().boxed();
}
