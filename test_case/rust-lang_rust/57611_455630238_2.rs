
error[E0631]: type mismatch in closure arguments
   --> spirit-tokio/examples/hws-tokio.rs:100:67
    |
96  |     let init = spirit_tokio::handlers::HandleListenerInit(|_, _| (), |conn, _| handle_connection(conn));
    |                                                                      --------------------------------- found signature of `fn(spirit_tokio::net::limits::LimitedConn<tokio::net::TcpStream>, _) -> _`
...
100 |         .with(Pipeline::new("listen").extract_cfg(Config::listen).transform(init).check())
    |                                                                   ^^^^^^^^^ expected signature of `for<'r> fn(spirit_tokio::net::limits::LimitedConn<tokio::net::TcpStream>, &'r mut _) -> _`
    |
