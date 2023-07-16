rust
async fn test2() -> impl IntoResponse {
    "test"
}

fn foo() {
    let foo: Option<&'_ dyn Fn()> = None;
}

async fn test(request: Request<Body>) -> impl IntoResponse {
    foo();
    test2().await
}
