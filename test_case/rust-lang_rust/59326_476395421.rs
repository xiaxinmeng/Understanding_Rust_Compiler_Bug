rust
use std::marker::PhantomData;

use bytes::Bytes; // 0.4.11;

trait Service {
    type Request;
    type Response;
}

trait Framing {
    type Request;
    type Response;
}

trait HttpService<B>: Service<Request = B::Request, Response = B::Response>
where
    B: Framing + Send + 'static,
{
    type Handler;
}

pub struct Http2Transport;
impl Framing for Http2Transport {
    type Request = Bytes;
    type Response = Bytes;
}

type BoxService<H> = Box<
    HttpService<Http2Transport, Handler = H, Request = Bytes, Response = Bytes> + Send + 'static,
>;

fn build_server<F, SVC, H>(factory: F)
where
    F: FnOnce() -> SVC + Send + Sync + Clone + 'static,
    SVC: Fn(i32) -> Result<BoxService<H>, std::io::Error>,
    H: 'static,
{
}

pub trait MyAwesomeService: Send + Sync + 'static {}

struct MyServiceImpl;
impl MyAwesomeService for MyServiceImpl {}

trait ProtocolFactory {
    type Frame: Framing;
}

pub struct Http2ProtocolFactory<F> {
    _phantom: PhantomData<F>,
}

impl<F> ProtocolFactory for Http2ProtocolFactory<F>
where
    F: Framing,
{
    type Frame = F;
}

pub struct ServiceProcessor<PF, H> {
    service: H,
    _phantom: PhantomData<(PF, H)>,
}

impl<PF, H> ServiceProcessor<PF, H>
where
    PF: ProtocolFactory + 'static,
    H: MyAwesomeService,
{
    pub fn new(service: H) -> Self {
        Self {
            service,
            _phantom: PhantomData,
        }
    }
}

impl<PF, H> Service for ServiceProcessor<PF, H>
where
    PF: ProtocolFactory + 'static,
    PF::Frame: Send + 'static,
    H: MyAwesomeService,
{
    type Request = <<PF as ProtocolFactory>::Frame as Framing>::Request;
    type Response = <<PF as ProtocolFactory>::Frame as Framing>::Response;
}

impl<PF, H> HttpService<PF::Frame> for ServiceProcessor<PF, H>
where
    PF: ProtocolFactory + 'static,
    PF::Frame: Send + 'static,
    H: MyAwesomeService,
{
    type Handler = H;
}

fn make_server<F, H>(
    proto: i32,
    handler: H,
) -> Result<
    Box<
        HttpService<
                F,
                Handler = H,
                Request = <F as Framing>::Request,
                Response = <F as Framing>::Response,
            > + Send
            + 'static,
    >,
    std::io::Error,
>
where
    F: Framing + Send + 'static,
    H: MyAwesomeService,
{
    Ok(Box::new(
        ServiceProcessor::<Http2ProtocolFactory<F>, H>::new(handler),
    ))
}

fn main() {
    build_server(|| |i: i32| make_server(i, MyServiceImpl {}))
}
