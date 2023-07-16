rust
async fn test(request: Request<Body>) -> impl IntoResponse {
    {
        let foo: Option<&'_ (dyn Fn() + Send)> = None;
        let _ = test2().await;
    }
    
    "test"
}