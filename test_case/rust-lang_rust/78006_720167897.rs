
$ cat tcp.rs
//! [`TcpStream::write`](std::net::TcpStream::write)
//! [`TcpStream::read`](std::net::TcpStream::read)
use std::io::{Read, Write};
$ rustdoc +rustc2 tcp.rs  # no output
