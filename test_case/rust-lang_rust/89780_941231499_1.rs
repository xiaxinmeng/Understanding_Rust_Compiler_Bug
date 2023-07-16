rust
fn some_future_impl() -> Poll<Result<(), ()>> {
    ready!(some_check())?;
    Poll::Ready(Ok(()))
}

fn some_stream_impl() -> Poll<Option<Result<(), ()>>> {
    ready!(some_check())?;
    Poll::Ready(Some(Ok(())))
}
