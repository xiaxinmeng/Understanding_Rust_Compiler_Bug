rust
// Must reside in a non-async context, e.g. a function returning [BoxFuture]
axum_server::bind(addr).serve(service).map_err(From::from).boxed()
