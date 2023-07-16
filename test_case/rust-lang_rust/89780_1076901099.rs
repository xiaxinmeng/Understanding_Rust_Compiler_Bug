rust
fn poll_next(...) -> Poll<Option<Result<T, E>>> {   // and also `poll(...) -> Poll<Result<T, E>>`
    // Return early with errors, don't propagate pending
    let ...: Poll<T> = some_future.poll(cx)?
    let ...: Poll<Option<T>> = some_child_stream.poll_next(cx)?;
    // Return early on either errors or pending
    let ...: T = ready!(some_future.poll(cx))?;
    let ...: Option<T> = ready!(some_child_stream.poll_next(cx))?;
    ...
}
