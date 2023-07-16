
error[E0053]: method `accept` has an incompatible type for trait
  --> unmp-link-udp\src\server.rs:86:5
   |
86 |     async fn accept(&mut self) -> Result<Connection, RecvError> {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected trait `Future<Output = Result<unmp_link::connection::Connection, unmp_link::error::RecvError>>`, found trait `Future<Output = Result<unmp_link::connection::Connection, unmp_link::error::RecvError>> + Send`
   |
   = note: expected fn pointer `fn(&'life0 mut server::Server) -> Pin<Box<(dyn Future<Output = Result<unmp_link::connection::Connection, unmp_link::error::RecvError>> + 'async_trait)>>`
              found fn pointer `fn(&'life0 mut server::Server) -> Pin<Box<(dyn Future<Output = Result<unmp_link::connection::Connection, unmp_link::error::RecvError>> + Send + 'async_trait)>>`

For more information about this error, try `rustc --explain E0053`.
error: could not compile `unmp-link-udp` due to previous error
