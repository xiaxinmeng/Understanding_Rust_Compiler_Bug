rust
fn some_stream_impl() -> Poll<Option<Result<(), ()>>> {
    some_check()?.ready()?;
    Poll::Ready(Some(Ok(())))
}

// and 

fn some_stream_impl() -> Poll<Option<Result<(), ()>>> {
    some_check().ready()??;
    Poll::Ready(Some(Ok(())))
}
