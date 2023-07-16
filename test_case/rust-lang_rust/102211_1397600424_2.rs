rust
async move {
    axum_server::bind(addr).serve(service).map_err(From::from).boxed().await
}.boxed() // This box fails to prove `Send` despite the prior succeeding
