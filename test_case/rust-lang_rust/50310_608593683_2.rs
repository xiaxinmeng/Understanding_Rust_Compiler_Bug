
error[E0271]: type mismatch resolving `<impl futures_core::stream::TryStream as futures_core::stream::Stream>::Item == std::result::Result<(bytes::bytes::Bytes, std::net::SocketAddr), std::io::Error>`
   --> src/main.rs:220:13
    |
220 |     let _ = task::spawn(foo);
    |             ^^^^^^^^^^^ expected associated type, found enum `std::result::Result`
    |
    = note: expected associated type `<impl futures_core::stream::TryStream as futures_core::stream::Stream>::Item`
                          found enum `std::result::Result<(bytes::bytes::Bytes, std::net::SocketAddr), std::io::Error>`
    = note: consider constraining the associated type `<impl futures_core::stream::TryStream as futures_core::stream::Stream>::Item` to `std::result::Result<(bytes::bytes::Bytes, std::net::SocketAddr), std::io::Error>` or calling a method that returns `<impl futures_core::stream::TryStream as futures_core::stream::Stream>::Item`
    = note: for more information, visit https://doc.rust-lang.org/book/ch19-03-advanced-traits.html
    = note: required because of the requirements on the impl of `core::future::future::Future` for `futures_util::stream::stream::forward::Forward<impl futures_core::stream::TryStream, futures_util::sink::map_err::SinkMapErr<futures_util::sink::fanout::Fanout<futures_channel::mpsc::UnboundedSender<(bytes::bytes::Bytes, std::net::SocketAddr)>, futures_channel::mpsc::UnboundedSender<(bytes::bytes::Bytes, std::net::SocketAddr)>>, [closure@src/main.rs:218:47: 218:101]>>`
