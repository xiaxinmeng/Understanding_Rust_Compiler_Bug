rust
pub type Poll<T, E> = Result<Async<T>, E>;
pub enum Async<T> {
    Ready(T),
    NotReady,
}

pub trait Future {
    type Item;
    type Error;
    fn poll(&mut self) -> Poll<Self::Item, Self::Error>;
}

pub trait IntoFuture {
    type Future: Future<Item=Self::Item, Error=Self::Error>;
    type Item;
    type Error;
    fn into_future(self) -> Self::Future;
}
impl<F: Future> IntoFuture for F {
    type Future = F;
    type Item = F::Item;
    type Error = F::Error;

    fn into_future(self) -> F {
        self
    }
}
impl<T, E> IntoFuture for Result<T, E> {
    type Future = FutureResult<T, E>;
    type Item = T;
    type Error = E;

    fn into_future(self) -> FutureResult<T, E> {
        FutureResult { inner: Some(self) }
    }
}

pub struct FutureResult<T, E> {
    inner: Option<Result<T, E>>,
}
impl<T, E> Future for FutureResult<T, E> {
    type Item = T;
    type Error = E;

    fn poll(&mut self) -> Poll<T, E> {
        self.inner.take().unwrap().map(Async::Ready)
    }
}


pub struct MapErr<A> where A: Future {
    future: A,
}
impl<A: Future> Future for MapErr<A>
{
    type Item = A::Item;
    type Error = Error;

    fn poll(&mut self) -> Poll<A::Item, Error> {
        let e = self.future.poll();
        e.map_err(|_| Error(::std::io::Error::last_os_error()))
    }
}


pub struct Select<A> where A: Future {
    inner: Option<A>,
}

impl<A> Future for Select<A>
    where A: Future,
{
    type Item = (A::Item, A);
    type Error = (A::Error, A);

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let mut a = self.inner.take().unwrap();
        let ret = match a.poll() {
            Err(a) => Err(a),
            Ok(Async::Ready(a)) => Ok(a),
            Ok(Async::NotReady) => loop {},
        };
        let next = a;
        match ret {
            Ok(a) => Ok(Async::Ready((a, next))),
            Err(e) => Err((e, next)),
        }
    }
}



pub enum Chain<A, B> {
    First(A),
    Second(B),
}
impl<A, B> Future for Chain<A, B>
    where A: Future, B: Future,
{
    type Item = B::Item;
    type Error = B::Error;
    fn poll(&mut self) -> Poll<B::Item, B::Error> {
        match *self {
            Chain::First(ref mut a) => {
                match a.poll() {
                    Ok(Async::Ready(t)) => t,
                    _ => loop {},
                }
            }
            Chain::Second(ref mut b) => return b.poll(),
        };
        loop {}
    }
}



pub struct Error(::std::io::Error);

struct Dummy<T>(T);
impl<T> Future for Dummy<T> {
    type Item = T;
    type Error = Error;
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {}
    }
}

const LENGTH: usize = 6144;

fn create() -> MapErr<
    Chain<
        Select<
            Chain<
                Dummy<[u8; LENGTH]>,
                FutureResult<i32, Error>
            >
        >,
        FutureResult<
            (),
            (
                Error,
                Chain<
                    Dummy<[u8; LENGTH]>,
                    FutureResult<i32, Error>
                >,
            )
        >
    >
>
{
    loop {}
}

pub fn run() -> Box<Future<Item = (), Error = Error>> {
    Box::new(create())
}
