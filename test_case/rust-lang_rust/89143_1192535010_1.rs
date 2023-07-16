rs
error[E0277]: the trait bound `mio::net::TcpStream: amiquip::IoStream` is not satisfied
   --> x/src/main.rs:68:26
    |
68  |     let mut connection = Connection::open_tls_stream(
    |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `amiquip::IoStream` is not implemented for `mio::net::TcpStream`
    |
    = help: the following other types implement trait `amiquip::IoStream`:
              amiquip::stream::native_tls::TlsStream<S>
              mio::net::tcp::TcpStream
note: required by a bound in `amiquip::Connection::open_tls_stream`
   --> /home/x/.cargo/registry/src/github.com-1ecc6299db9ec823/amiquip-0.4.2/src/connection.rs:262:66
    |
262 |     pub fn open_tls_stream<Auth: Sasl, C: Into<TlsConnector>, S: IoStream>(
    |                                                                  ^^^^^^^^ required by this bound in `amiquip::Connection::open_tls_stream`

For more information about this error, try `rustc --explain E0277`.
