rust
#![feature(futures_api, pin, async_await, await_macro, arbitrary_self_types)]
#![feature(trait_alias)]

use core::pin::Pin;
use futures::channel::mpsc;
use futures::{future, Future};

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

pub type ConnPair<SendItem, RecvItem> = (mpsc::Sender<SendItem>,mpsc::Receiver<RecvItem>);

/// Transform a connection into another connection
pub trait ConnTransform {
    type OldSendItem;
    type OldRecvItem;
    type NewSendItem;
    type NewRecvItem;
    type Arg;

    fn transform(&mut self, arg: Self::Arg, conn_pair: ConnPair<Self::OldSendItem, Self::OldRecvItem>) 
        -> BoxFuture<'_, Option<ConnPair<Self::NewSendItem, Self::NewRecvItem>>>;
}

pub trait BytesConnTransform<Arg> = ConnTransform<OldSendItem=Vec<u8>,OldRecvItem=Vec<u8>,
                                                 NewSendItem=Vec<u8>,NewRecvItem=Vec<u8>,
                                                 Arg=Arg>;


