
error[E0631]: type mismatch in closure arguments
   --> src/future.rs:298:31
    |
294 |             let mut read_line = |_context| -> Poll<String> {
    |                                 -------------------------- found signature of `fn(_) -> _`
...
298 |             let read_future = futures::future::poll_fn(read_line);
    |                               ^^^^^^^^^^^^^^^^^^^^^^^^ expected signature of `for<'r, 's> fn(&'r mut std::task::Context<'s>) -> _`
    |
    = note: required by `futures_util::future::poll_fn::poll_fn`
