rust
use futures::{try_ready, Async, Future, Poll};
use serde::de::DeserializeOwned;
use tokio::io::{read_exact, AsyncRead};

use std::{io, marker::PhantomData};

#[derive(Debug)]
pub struct Bar<R> {
    stream: R,
}


impl<R> Future for Bar<R>
where
    R: AsyncRead,
{
    type Item = Foo;
    type Error = io::Error;
    fn poll(&mut self) -> Poll<Self::Item, io::Error> {
        let mut buf = [0_u8; 14];
// if you comment out the following line, error goes away:
       let (rd, init) = try_ready!(read_exact(&self.stream, &mut buf).poll());

        Ok(Async::Ready(Foo { }))
    }
}

pub struct Foo {}
