rust
let f: std::pin::Pin<Box<std::future::Future<Output = _>>> = Box::pin(fetched.build_dependencies());
let deps_built = await!(f);
