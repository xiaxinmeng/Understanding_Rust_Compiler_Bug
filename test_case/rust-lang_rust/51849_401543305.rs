
failures:
---- net::tcp::tests::connect_timeout_unbound stdout ----
thread 'net::tcp::tests::connect_timeout_unbound' panicked at 'called `Result::unwrap_err()` on an `Ok` value: TcpStream { addr: V4(127.0.0.1:3605), peer: V4(127.0.0.1:3605), socket: 728 }', libcore\result.rs:945:5
failures:
    net::tcp::tests::connect_timeout_unbound
