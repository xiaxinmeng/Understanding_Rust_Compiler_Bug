rust 
use std::fmt::Debug;

fn main() {
    let mut log_service = LogService { inner: Inner };
    log_service.call(Struct1 {});
}

pub trait Service<Request> {
    type Response;

    fn call(&mut self, req: Request) -> Self::Response;
}

pub struct LogService<S> {
    inner: S,
}

impl<T, S, U> Service<T> for LogService<S>
where
    S: Service<T, Response = U>,
    U: for<'a> Extension<'a>,
    for<'a> <U as Extension<'a>>::Item: std::fmt::Debug,
{
    type Response = S::Response;

    fn call(&mut self, req: T) -> Self::Response {
        self.inner.call(req)
    }
}

pub struct Inner;

impl Service<Struct1> for Inner {
    type Response = Resp;

    fn call(&mut self, req: Struct1) -> Self::Response {
        Resp::A(req)
    }
}

pub trait Extension<'a> {
    type Item;
}

pub trait E2
where
    Self: for<'a> Extension<'a>,
{
    fn touch<F>(self, f: F) -> Self
    where
        F: for<'b> Fn(<Self as Extension<'b>>::Item);
}

#[derive(Debug)]
pub struct Struct1 {}

pub enum Resp {
    A(Struct1),
}

impl<'a> Extension<'a> for Resp {
    type Item = RespItem<'a>;
}

impl E2 for Resp {
    fn touch<F>(self, f: F) -> Self
    where
        F: for<'b> Fn(<Self as Extension<'b>>::Item),
    {
        match &self {
            Resp::A(a) => f(RespItem::A(a)),
        }
        self
    }
}

pub enum RespItem<'a> {
    A(&'a Struct1),
}

impl Debug for RespItem<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A(arg0) => f.debug_tuple("A").field(arg0).finish(),
        }
    }
}
