rust
use axum::{
    body::Body,
    http::Request,
    response::IntoResponse,
    Router,
    routing::get,
};
use tower::util::ServiceExt;
use tower_http::services::ServeFile;

async fn test(request: Request<Body>) -> impl IntoResponse {
    let foo: Option<&'_ dyn Fn()> = None;
    ServeFile::new("index.htm").oneshot(request).await.unwrap()
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(test));
}
