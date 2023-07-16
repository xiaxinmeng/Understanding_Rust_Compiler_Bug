
use std::marker::PhantomData;
use std::io;

use tokio::io::{AsyncRead, AsyncWrite};
use tokio::net::TcpStream;
use tokio_util::codec::{Encoder, Decoder};
use futures::stream::{Stream, StreamExt};

use bytes::BytesMut;

struct Codec;

struct Message<'a> { _p: PhantomData<&'a u8> }

impl Encoder<Message<'static>> for Codec {
    type Error = io::Error;
    
    fn encode(&mut self, item: Message<'static>, dst: &mut BytesMut) -> Result<(), Self::Error> {
        todo!()
    }
}

impl Decoder for Codec {
    type Item = Message<'static>;
    type Error = io::Error;
    
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        todo!()
    }
}

fn coerce(s: impl Stream<Item=io::Result<Message<'static>>>) -> impl Stream<Item=io::Result<Message<'static>>> { s }

async fn forwarder(sock: impl AsyncRead + AsyncWrite + Send + 'static) {
    let sock = Codec.framed(sock);
    let (sink, stream) = sock.split();
    
    coerce(stream.map(|msg| msg)).forward(sink).await;
}

#[allow(unreachable_code)]
pub fn assert_send() {
    let s : TcpStream = todo!();
    // The diagnostic emitted here should be reported in the body of forwarder itself.
    let _b : Box<dyn Send> = Box::new(forwarder(s));
}
