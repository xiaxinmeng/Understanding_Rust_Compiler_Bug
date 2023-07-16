rust
fn bind_service<S, F>()
where
    S: for<'a> Service<&'a (), Future = F>,
