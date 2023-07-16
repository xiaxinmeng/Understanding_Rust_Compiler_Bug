rust
use std::convert::Infallible;

async fn smarvice() -> Result<hyper::http::Response<hyper::Body>, Infallible> {
    ident_error;
    todo!()
}

#[allow(dead_code)]
async fn iceice<A, B>(bldr: hyper::server::Builder<hyper::server::conn::AddrIncoming>) 
where
    A: Send,
    B: Send,
{
    let service = hyper::service::make_service_fn(|_| {
        async {
            Ok::<_, Infallible>(hyper::service::service_fn(|_| {
                smarvice()
            }))
        }
    });

    bldr.serve(service).await.unwrap();
}

fn main() {}
