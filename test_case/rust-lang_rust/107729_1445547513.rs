rust
fn svc() -> impl Service<Request<Body>, Response = Response<Body>, Error = Error, Future = impl Send> {
  // ...
}
