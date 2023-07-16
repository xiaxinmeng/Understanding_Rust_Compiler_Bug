rust
async fn index() -> Result<impl Reply, Rejection> { Ok(warp::reply::html("")) }
pub async fn create_warp_server() -> Result<impl Future<Output = ()> + 'static, Box<dyn std::error::Error>> {
    let index = get().and_then(crate::index);
    let cors = allow_origins(vec!["http://localhost:8080"].into_iter());
    let routes = index.with(cors);
    let server = serve(routes).bind_with_graceful_shutdown();
    Ok(server)
}
fn main() {
    let runtime = Builder::new_current_thread().build().unwrap();
    let server = runtime.block_on(create_warp_server()).unwrap();
    runtime.block_on(server);
}
