Rust

#![feature(await_macro, async_await, futures_api, pin,)]

#[macro_use]
extern crate tokio;

use hyper::Uri;

use hyper::Client;

use hyper::client::HttpConnector;
use hyper::rt::Stream as HStream;

use futures::sync::mpsc::*;
use futures::Future;

use tokio::prelude::{Sink, Stream};

use std::borrow::ToOwned;

async fn read_url<'a>(
    url: Uri,
    sender: UnboundedSender<Vec<u8>>,
    client: &'a Client<HttpConnector, hyper::Body>,
) {
    await!(
        client
            .get(url)
            .and_then(|content| content.into_body().for_each(|item| {
                sender.start_send(vec![]);
                Ok(())
            }))
    );
}

fn main() {
    let (sender, recv) = unbounded::<Vec<u8>>();

    let url_sender = sender.clone();
    let client = Client::new();

    tokio::run_async(read_url(
        "http://ikariam.org".parse().unwrap(),
        url_sender,
        &client,
    ));
}
