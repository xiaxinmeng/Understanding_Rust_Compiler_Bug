rust
fn some_future_impl() -> Poll<Result<(), ()>> {
    some_check().ready()??;
    Poll::Ready(Ok(()))
}

fn some_stream_impl() -> Poll<Option<Result<(), ()>>> {
    some_check().ready()??;
    Poll::Ready(Some(Ok(())))
}
