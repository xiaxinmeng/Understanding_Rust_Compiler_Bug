rust
trait SendFuture: Send {
    type Output;
}

impl<Fut: Send> SendFuture for Fut {
    type Output = ();
}

async fn broken_fut() {
    ident_error;
}

// triggers normalization of `<Fut as SendFuture>::Output`,
// which requires `Fut: Send`.
fn normalize<Fut: SendFuture>(_: Fut, _: Fut::Output) {}

async fn iceice<A, B>() // <- async fn is necessary
where
    A: Send,
    B: Send, // <- a second bound
{
    normalize(broken_fut(), ());
}
