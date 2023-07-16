rust
#![allow(unreachable_code)]
extern crate futures;

use futures::future::{join_all, poll_fn, done, ok};
use futures::future::Future;
use futures::future::Either;
use futures::Async;

fn main() {
    let _b: Box<Future<Item = _, Error = _> + Send>;
    _b = Box::new(graphql());
}

struct BlockingError;
struct Request;
struct RequestError;
struct RootNode2<'a> { _schema: &'a (), }

fn graphql() -> impl Future<Item = Vec<()>, Error = BlockingError>
{
    match () {
        _ if false => Either::A(done(Err(RequestError)).or_else(|_| ok(vec![]))),
        _ => Either::B(
            done(Err(RequestError))
                .and_then(|_: Request| join_all(Some(&unimplemented!()).into_iter()
                                                .map(|root_node: &RootNode2<'static>| {
                                                    poll_fn(|| {
                                                        &root_node;
                                                        Ok(Async::Ready(()))
                                                    })
                                                })))
                .or_else(|_| ok(vec![]))),
    }
}
