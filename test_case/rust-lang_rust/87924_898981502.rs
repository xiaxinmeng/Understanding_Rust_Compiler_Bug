rust
 let app = route("/hello", get(handler))
        .nest(
            "/",
            axum::service::get(ServeDir::new("./publish").append_index_html_on_directories(true))
                .handle_error(|error: std::io::Error| {
                    Ok::<_, std::convert::Infallible>((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", error),
                    ))
                }),
        )
        .nest(
            "/upload",
            axum::service::get(ServeDir::new("./upload")).handle_error(|error: std::io::Error| {
                Ok::<_, std::convert::Infallible>((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                ))
            }),
        )
        .nest(
            "/api",
            route(
                "/upload/image",
                post(handler),
            )
            .nest(
                "/pay",
                route("/get_pay_params", post(handler))
                    .route("/transfer", post(handler))
                    .boxed(),
            )
            .nest(
                "/user",
                route("/list", get(handler))
                    .route("/create", post(handler))
                    .route("/login", post(handler))
                    .route("/info", get(handler))
                    .route("/update_password", get(handler))
                    .route("/money", get(handler))
                    .boxed(),
            )
            .nest(
                "/order",
                route("/create", post(handler))
                    .route("/list", get(handler))
                    .boxed(),
            )
            .nest(
                "/product",
                route("/list", get(handler))
                    .route("/home", get(handler))
                    .route("/create", post(handler))
                    .route("/update/:id", post(handler))
                    .route("/delete", post(handler))
                    .route("/detail", get(handler))
                    .boxed(),
            )
            .boxed(),
        )
        .layer(
            ServiceBuilder::new()
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .layer(AddExtensionLayer::new(pool))
                .into_inner(),
        );
