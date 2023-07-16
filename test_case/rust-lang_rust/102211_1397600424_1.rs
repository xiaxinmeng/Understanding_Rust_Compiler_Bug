rust
async move {
    axum_server::bind(addr).serve(service).map_err(From::from).await
}.boxed()
