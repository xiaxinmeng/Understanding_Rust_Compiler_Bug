rust
#![feature(allocator_api)]

use std::net::SocketAddr;

use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::Response;
use hyper::body::Bytes;
use tokio::net::TcpListener;

async fn handle_req() -> Result<Response<BoxBody<Bytes, hyper::Error>>, Box<dyn std::error::Error + Send + Sync>> {
    todo!()
}

pub async fn start_web_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    loop {
        let (stream, _) = listener.accept().await?;
        tokio::task::spawn(async move {
            http1::Builder::new()
                .serve_connection(stream, service_fn(|_| handle_req()))
                .await;
        });
    }
}

fn main() {}
