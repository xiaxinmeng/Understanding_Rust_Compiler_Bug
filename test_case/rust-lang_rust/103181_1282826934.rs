rust
use hyper::http;

use std::{convert::Infallible, future::Future, net::SocketAddr, sync::Arc};

async fn request_handler(jvs_file: &str) -> http::Response<hyper::Body> {
    todo!()
}

async fn service<H, F>(r_handler: Arc<H>) -> Result<http::Response<hyper::Body>, Infallible>
where
    H: Fn(&str) -> F + Send,
    F: Future<Output = http::Response<hyper::Body>> + Send,
{
    r_handler(s).await;
    todo!()
}

async fn serve<H, F>(socket: SocketAddr) -> hyper::Result<()>
where
    H: Fn(&str) -> F + Send,
    F: Future<Output = http::Response<hyper::Body>> + Send,
{
    let request_handler = Arc::new(request_handler);
    let service = hyper::service::make_service_fn(|_| {
        let request_handler = request_handler.clone();
        async move {
            Ok::<_, Infallible>(hyper::service::service_fn(move |_| {
                let request_handler = request_handler.clone();
                service(request_handler)
            }))
        }
    });

    let server = hyper::server::Server::try_bind(&socket)?;
    server.serve(service).await?;

    Ok(())
}

fn main() {}
