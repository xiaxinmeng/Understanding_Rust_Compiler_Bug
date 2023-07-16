rust
async fn test(request: Request<Body>) -> impl IntoResponse {
    {
        let foo: Option<&'_ (dyn Fn() + Send)> = None;
    }
    
    test2().await
}
