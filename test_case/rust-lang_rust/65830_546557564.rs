
failures:

---- net::tcp::tests::double_bind stdout ----
thread '<unnamed>' panicked at 'This system (perhaps due to options set by TcpListener::bind) permits double binding: TcpListener { addr: V4(127.0.0.1:19613), fd: 8 } and TcpListener { addr: V4(127.0.0.1:19613), fd: 15 }', src/libstd/net/tcp.rs:1257:34


failures:
    net::tcp::tests::double_bind

test result: FAILED. 762 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
