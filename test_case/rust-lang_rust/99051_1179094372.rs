rust
use axum::{
    body::Body,
    http::Request,
    response::IntoResponse,
    Router,
    routing::get,
};

async fn test2() -> impl IntoResponse {
    "test"
}

async fn test(request: Request<Body>) -> impl IntoResponse {
    let foo: Option<&'_ (dyn Fn() + Send)> = None;
    test2().await
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(test));
}
