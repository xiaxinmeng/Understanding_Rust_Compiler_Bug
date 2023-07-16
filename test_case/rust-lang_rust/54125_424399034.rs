
[00:52:09] error: unused variable: `b`
[00:52:09]    --> libstd/net/tcp.rs:732:34
[00:52:09]     |
[00:52:09] 732 |         self.0.accept().map(|(a, b)| (TcpStream(a), b))
[00:52:09]     |                                  ^ help: consider using `_b` instead
[00:52:09]     = note: `-D unused-variables` implied by `-D warnings`
