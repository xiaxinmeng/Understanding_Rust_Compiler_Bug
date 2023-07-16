rust
use std::future::{pending, Future};

async fn handle_request() -> Result<(), String> {
    loop {}
}

struct Server<S> {
    _make_service: S,
}

impl Server<()> {
    pub fn bind<S>(_make_service: S) -> Server<S> {
        loop {}
    }
}
impl<T> Server<T> {
    fn with_graceful_shutdown<>(self) -> impl Future<Output = ()>
    {
        pending()
    }
}

pub(crate) async fn create_and_bind(
    _: impl Future<Output = ()>,
) -> (impl Future<Output = ()>,) {
    let server = Server::bind(handle_request().await);

    (async { server.with_graceful_shutdown().await; },)
}

pub async fn new() -> Result<(), String> {
    let (_future,) = create_and_bind(async { pending().await }).await;

    Ok(())
}
