rust
pub fn build_app(pool: Arc<Pool>) -> BoxRoute<Body, Box<(dyn std::error::Error + Send + Sync + 'static)>> {
    let middleware_stack = ServiceBuilder::new()
        .timeout(Duration::from_secs(10))
        .concurrency_limit(100)
        .layer(TraceLayer::new_for_http())
        .layer(MapResponseLayer::new(map_errors))
        .layer(AddExtensionLayer::new(pool))
        .into_inner();

    route("/users/:user_id", get(users::resource::get).delete(users::resource::delete))
        .route("/users/:user_id/connections/", get(connections::collection::get))
        .route("/users/", get(users::collection::get).post(users::collection::post))
        .route("/organisations/:org_id", get(organisations::resource::get).delete(organisations::resource::delete))
        .route("/organisations/", get(organisations::collection::get).post(organisations::collection::post))
        .route("/organisations/:org_id/connections/:user_id", get(connections::resource::get).delete(connections::resource::delete))
        .route("/organisations/:org_id/connections/", get(connections::resource::get).post(connections::resource::post))
        .layer(middleware_stack)
        .boxed()
}
