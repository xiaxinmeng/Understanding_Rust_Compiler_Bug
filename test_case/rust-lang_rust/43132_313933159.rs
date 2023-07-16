rust
extern crate futures;
extern crate futures_mpsc;
extern crate tk_http;
extern crate tk_listen;
extern crate tokio_core;

use std::sync::Arc;

use futures::{Future, Sink, Stream};
use futures::future::FutureResult;
use futures::stream::empty as empty_stream;
use tokio_core::reactor::Core;
use tk_http::server::buffered::{Request, BufferedDispatcher};
use tk_http::server::{Encoder, EncoderDone, Config, Proto, Error};
use tk_listen::ListenExt;

use tokio_core::net::TcpStream;
use std::net::SocketAddr;

fn http<S>(_: Request, _: Encoder<S>)
    -> FutureResult<EncoderDone<S>, Error>
{
    unimplemented!()
}

fn main() {
    let h = Core::new().unwrap().handle();

    let (sender, receiver) = futures_mpsc::channel::<()>(1);
    let sender = Arc::new(sender);

    let _ = empty_stream::<(TcpStream, SocketAddr), ()>()
        .map(|(socket, addr)| {
            let sender = sender.clone();
            Proto::new(socket, &Config::new().done(),
                BufferedDispatcher::new_with_websockets(addr, &h,
                    http,
                    move |_, _| {
                        let sender = (*sender).clone();
                        futures::future::ok(()).and_then(move |_| {
                            empty_stream::<(), String>().fold(sender, |sender, _| {
                                sender.send(()).map_err(|_| "")
                            })
                        })
                        .then(|_| Ok(()))
                    }),
                &h)
            .then(|_| Ok(()))
        })
        .listen(1000)
        .join(receiver.for_each(|_| Ok(())));
}
