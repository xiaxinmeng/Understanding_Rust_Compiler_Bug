
error[E0599]: the method `say_hello` exists for struct `Greeter<RequestModifier<Connection<TcpStream, DefaultExecutor, BoxBody>, ()>>`, but its trait bounds were not satisfied
   |
  ::: .../tower-error-example/target/debug/build/tower-error-example-7aa5e3a34e8c258f/out/helloworld.rs:19:5
   |
19 |     pub struct Greeter<T> {
   |     --------------------- method `say_hello` not found for this struct
  --> src/main.rs:35:18
   |
35 |                 .say_hello(Request::new(HelloRequest {
   |                  ^^^^^^^^^ method cannot be called due to unsatisfied trait bounds
   |
  ::: .../tower-h2-aaf68b3c8982de4c/a3c958a/src/client/connection.rs:18:1
   |
18 | pub struct Connection<T, E, S>
   | ------------------------------ doesn't satisfy `_: Service<Request<()>>`
   |
  ::: .../tower-http-8b33967e083bb0de/e7ef6ef/tower-request-modifier/src/lib.rs:13:1
   |
13 | pub struct RequestModifier<T, B> {
   | --------------------------------
   | |
   | doesn't satisfy `<_ as Service<Request<()>>>::Response = Response<_>`
   | doesn't satisfy `_: Service<Request<()>>`
   |
   = note: the following trait bounds were not satisfied:
           `<RequestModifier<tower_h2::client::Connection<tokio::net::TcpStream, DefaultExecutor, BoxBody>, ()> as tower_service::Service<http::Request<()>>>::Response = http::Response<_>`
           `RequestModifier<tower_h2::client::Connection<tokio::net::TcpStream, DefaultExecutor, BoxBody>, ()>: tower_service::Service<http::Request<()>>`
           `tower_h2::client::Connection<tokio::net::TcpStream, DefaultExecutor, BoxBody>: tower_service::Service<http::Request<()>>`
