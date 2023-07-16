rust
fn poll_next(...) -> Poll<Option<Result<T, E>>> {   // and also `poll(...) -> Poll<Result<T, E>>`
    // Return early with errors, don't propagate pending
    let ...: Poll<T> = some_future.poll(cx).bikeshed_transpose_result()?
    let ...: Poll<Option<T>> = some_child_stream.poll_next(cx).bikeshed_transpose_result()?;
    // Return early on either errors or pending
    let ...: T = some_future.poll(cx).ready()?;
    let ...: Option<T> = some_child_stream.poll_next(cx).ready()??;
    ...
}
