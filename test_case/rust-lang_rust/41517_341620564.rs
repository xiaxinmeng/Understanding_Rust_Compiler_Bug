rust
trait HttpService = Service<http::Request<Self::RequestBody>> {
    type RequestBody;
}
