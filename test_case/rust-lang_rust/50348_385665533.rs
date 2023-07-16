plain
[01:43:27] ................iiii........ii......................................................................
[01:43:38] ....................................................................................................
[01:43:58] ................................................................i...................................
[01:44:30] ....................................................................................................
[01:44:39] ........................F...................................FFFFFFFFFFFFFFFFFFFFFFFFF..............i
[01:44:50] iii...............................................
[01:44:50] 
[01:44:50] 
[01:44:50] ---- sys/unix/ext/net.rs - sys::unix::ext::net::IncomingSeqpacket (line 1729) stdout ----
[01:44:50]  error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]   --> sys/unix/ext/net.rs:1738:16
[01:44:50]    |
[01:44:50] 12 | let listener = UnixSeqpacketListener::bind("/path/to/the/socket").unwrap();
[01:44:50]    |
[01:44:50]    |
[01:44:50]    = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:1732:26
[01:44:50]   |
[01:44:50] 6 | use std::os::unix::net::{UnixSeqpacket, UnixSeqpacketListener};
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:1732:41
[01:44:50]   |
[01:44:50] 6 | use std::os::unix::net::{UnixSeqpacket, UnixSeqpacketListener};
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:1734:26
[01:44:50]   |
[01:44:50] 8 | fn handle_client(socket: UnixSeqpacket) {
[01:44:50] 
[01:44:50] 
[01:44:50] error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:1892:19
[01:44:50]   |
[01:44:50] 9 | let addr = socket.local_addr().expect("Couldn't get local address");
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] thread 'sys/unix/ext/net.rs - sys::unix::ext::net::UnixSeqpacket::local_addr (line 1886)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:44:50] 
[01:44:50] ---- sys/unix/ext/net.rs - sys::unix::ext::net::UnixSeqpacket::connect (line 1811) stdout ----
[01:44:50]  error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:1815:20
[01:44:50]   |
[01:44:50] 7 | let socket = match UnixSeqpacket::connect("/tmp/socket") {
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:1813:5
[01:44:50]   |
[01:44:50] 5 | use std::os::unix::net::UnixSeqpacket;
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] thread 'sys/unix/ext/net.rs - sys::unix::ext::net::UnixSeqpacket::connect (line 1811)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:44:50] 
[01:44:50] ---- sys/unix/ext/net.rs - sys::unix::ext::net::UnixSeqpacket (line 1777) stdout ----
[01:44:50]  error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:1782:14
[01:44:50]   |
[01:44:50] 8 | let socket = UnixSeqpacket::connect(path).unwrap();
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:1779:5
[01:44:50]   |
[01:44:50] 5 | use std::os::unix::net::UnixSeqpacket;
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:1783:21
[01:44:50]   |
[01:44:50] 9 | let _count = socket.send(b"hello world").unwrap();
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]   --> sys/unix/ext/net.rs:1785:20
[01:44:50]    |
[01:44:50] 11 | let count = socket.recv(&mut buf).unwrap();
[01:44:50]    |
[01:44:50]    |
[01:44:50]    = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] thread 'sys/unix/ext/net.rs - sys::unix::ext::net::UnixSeqpacket (line 1777)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:44:50] 
[01:44:50] ---- sys/unix/ext/net.rs - sys::unix::ext::net::UnixSeqpacket::pair (line 1843) stdout ----
[01:44:50]  error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:1847:28
[01:44:50]   |
[01:44:50] 7 | let (sock1, sock2) = match UnixSeqpacket::pair() {
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:1845:5
[01:44:50]   |
[01:44:50] 5 | use std::os::unix::net::UnixSeqpacket;
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] thread 'sys/unix/ext/net.rs - sys::unix::ext::net::UnixSeqpacket::pair (line 1843)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:44:50] 
[01:44:50] ---- sys/unix/ext/net.rs - sys::unix::ext::net::UnixSeqpacket::peer_addr (line 1903) stdout ----
[01:44:50]  error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:1907:14
[01:44:50]   |
[01:44:50] 7 | let socket = UnixSeqpacket::connect("/path/to/the/socket").unwrap();
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpac:50]   |
[01:44:50] 5 | use std::os::unix::net::UnixSeqpacket;
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:2048:8
[01:44:50]   |
[01:44:50] 9 | socket.set_read_timeout(Some(Duration::new(1, 0)))
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]   --> sys/unix/ext/net.rs:2050:19
[01:44:50]    |
[01:44:50] 11 | assert_eq!(socket.read_timeout().unwrap(), Some(Duration::new(1, 0)));
[01:44:50]    |
[01:44:50]    |
[01:44:50]    = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] thread 'sys/unix/ext/net.rs - sys::unix::ext::net::UnixSeqpacket::read_timeout (line 2042)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:44:50] 
[01:44:50] ---- sys/unix/ext/net.rs - sys::unix::ext::net::UnixSeqpacket::recv (line 1922) stdout ----
[01:44:50]  error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:1926:14
[01:44:50]   |
[01:44:50] 7 | let socket = UnixSeqpacket::connect("/path/to/the/socket").unwrap();
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:1924:5
[01:44:50]   |
[01:44:50] 5 | use std::os::unix::net::UnixSeqpacket;
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:1928:8
[01:44:50]   |
[01:44:50] 9 | socket.recv(buf.as_mut_slice()).expect("recv function failed");
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] thread 'sys/unix/ext/net.rs - sys::unix::ext::net::UnixSeqpacket::recv (line 1922)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:44:50] 
[01:44:50] ---- sys/unix/ext/net.rs - sys::unix::ext::net::UnixSeqpacket::send (line 1941) stdout ----
[01:44:50]  error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:1945:14
[01:44:50]   |
[01:44:50] 7 | let socket = UnixSeqpacket::connect("/path/to/the/socket").unwrap();
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:1943:5
[01:44:50]   |
[01:44:50] 5 | use std::os::unix::net::UnixSeqpacket;
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:1946:8
[01:44:50]   |
[01:44:50] 8 | socket.send(b"omelette au fromage").expect("send_to function failed");
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] thread 'sys/unix/ext/net.rs - sys::unix::ext::net::UnixSeqpacket::send (line 1941)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:44:50] 
[01:44:50] ---- sys/unix/ext/net.rs - sys::unix::ext::net::UnixSeqpacket::set_nonblocking (line 2080) stdout ----
[01:44:50]  error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:2084:14
[01:44:50]   |
[01:44:50] 7 | let socket = UnixSeqpacket::connect("/path/to/the/socket").unwrap();
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:2082:5
[01:44:50]   |
[01:44:50] 5 | use std::os::unix::net::UnixSeqpacket;
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:2085:8
[01:44:50]   |
[01:44:50] 8 | socket.set_nonblocking(true).expect("set_nonblocking function failed");
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] thread 'sys/unix/ext/net.rs - sys::unix::ext::net::UnixSeqpacket::set_nonblocking (line 2080)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:44:50] 
[01:44:50] ---- sys/unix/ext/net.rs - sys::unix::ext::net::UnixSeqpacket::set_read_timeout (line 1967) stdout ----
[01:44:50]  error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:1972:14
[01:44:50]   |
[01:44:50] 8 | let socket = UnixSeqpacket::connect("/path/to/the/socket").unwrap();
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:1969:5
[01:44:50]   |
[01:44:50] 5 | use std::os::unix::net::UnixSeqpacket;
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:1973:8
[01:44:50]   |
[01:44:50] 9 | socket.set_read_timeout(Some(Duration::new(1, 0)))
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] thread 'sys/unix/ext/net.rs - sys::unix::ext::net::UnixSeqpacket::set_read_timeout (line 1967)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:44:50] 
[01:44:50] ---- sys/unix/ext/net.rs - sys::unix::ext::net::UnixSeqpacket::set_write_timeout (line 2009) stdout ----
[01:44:50]  error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:2014:14
[01:44:50]   |
[01:44:50] 8 | let socket = UnixSeqpacket::connect("/path/to/the/socket").unwrap();
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:2011:5
[01:44:50]   |
[01:44:50] 5 | use std::os::unix::net::UnixSeqpacket;
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:2015:8
[01:44:50]   |
[01:44:50] 9 | socket.set_write_timeout(Some(Duration::new(1, 0)))
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] thread 'sys/unix/ext/net.rs - sys::unix::ext::net::UnixSeqpacket::set_write_timeout (line 2009)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:44:50] 
[01:44:50] ---- sys/unix/ext/net.rs - sys::unix::ext::net::UnixSeqpacket::set_read_timeout (line 1980) stdout ----
[01:44:50]  error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:1986:14
[01:44:50]   |
[01:44:50] 9 | let socket = UnixSeqpacket::connect("/path/to/the/socket").unwrap();
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:1983:5
[01:44:50]   |
[01:44:50] 6 | use std::os::unix::net::UnixSeqpacket;
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]   --> sys/unix/ext/net.rs:1987:21
[01:44:50]    |
[01:44:50] 10 | let result = socket.set_read_timeout(Some(Duration::new(0, 0)));
[01:44:50]    |
[01:44:50]    |
[01:44:50]    = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] thread 'sys/unix/ext/net.rs - sys::unix::ext::net::UnixSeqpacket::set_read_timeout (line 1980)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:44:50] 
[01:44:50] ---- sys/unix/ext/net.rs - sys::unix::ext::net::UnixSeqpacket::set_write_timeout (line 2022) stdout ----
[01:44:50]  error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:2028:14
[01:44:50]   |
[01:44:50] 9 | let socket = UnixSeqpacket::connect("/path/to/the/socket").unwrap();
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:2025:5
[01:44:50]   |
[01:44:50] 6 | use std::os::unix::net::UnixSeqpacket;
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]   --> sys/unix/ext/net.rs:2029:21
[01:44:50]    |
[01:44:50] 10 | let result = socket.set_write_timeout(Some(Duration::new(0, 0)));
[01:44:50]    |
[01:44:50]    |
[01:44:50]    = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] thread 'sys/unix/ext/net.rs - sys::unix::ext::net::UnixSeqpacket::set_write_timeout (line 2022)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:44:50] 
[01:44:50] ---- sys/unix/ext/net.rs - sys::unix::ext::net::UnixSeqpacket::shutdown (line 2118) stdout ----
[0o/the/socket").unwrap();
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:2098:5
[01:44:50]   |
[01:44:50] 5 | use std::os::unix::net::UnixSeqpacket;
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:2101:31
[01:44:50]   |
[01:44:50] 8 | if let Ok(Some(err)) = socket.take_error() {
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] thread 'sys/unix/ext/net.rs - sys::unix::ext::net::UnixSeqpacket::take_error (line 2096)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:44:50] 
[01:44:50] ---- sys/unix/ext/net.rs - sys::unix::ext::net::UnixSeqpacket::try_clone (line 1869) stdout ----
[01:44:50]  error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:1873:14
[01:44:50]   |
[01:44:50] 7 | let socket = UnixSeqpacket::connect("/path/to/the/socket").unwrap();
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:5::UnixSeqpacket;
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:2067:8
[01:44:50]   |
[01:44:50] 9 | socket.set_write_timeout(Some(Duration::new(1, 0)))
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]   --> sys/unix/ext/net.rs:2069:19
[01:44:50]    |
[01:44:50] 11 | assert_eq!(socket.write_timeout().unwrap(), Some(Duration::new(1, 0)));
[01:44:50]    |
[01:44:50]    |
[01:44:50]    = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] thread 'sys/unix/ext/net.rs - sys::unix::ext::net::UnixSeqpacket::write_timeout (line 2061)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:44:50] 
[01:44:50] ---- sys/unix/ext/net.rs - sys::unix::ext::net::UnixSeqpacketListener (line 1474) stdout ----
[01:44:50]  error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]   --> sys/unix/ext/net.rs:1483:16
[01:44:50]    |
[01:44:50] 12 | let listener = UnixSeqpacketListener::bind("/path/to/the/socket").unwrap();
[01:44:50]    |
[01:44:50]    |
[01:44:50]    = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:1477:26
[01:44:50]   |
[01:44:50] 6 | use std::os::unix::net::{UnixSeqpacket, UnixSeqpacketListener};
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:1477:41
[01:44:50]   |
[01:44:50] 6 | use std::os::unix::net::{UnixSeqpacket, UnixSeqpacketListener};
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:1479:26
[01:44:50]   |
[01:44:50] 8 | fn handle_client(socket: UnixSeqpacket) {
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]   --> sys/unix/ext/net.rs:1486:24
[01:44:50]    |
[01:44:50] 15 | for socket in listener.incoming() {
[01:44:50]    |
[01:44:50]    |
[01:44:50]    = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] thread 'sys/unix/ext/net.rs - sys::unix::ext::net::UnixSeqpacketListener (line 1474)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:44:50] 
[01:44:50] ---- sys/unix/ext/net.rs - sys::unix::ext::net::UnixSeqpacketListener::bind (line 1519) stdout ----
[01:44:50]  error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:1523:22
[01:44:50]   |
[01:44:50] 7 | let listener = match UnixSeqpacketListener::bind("/path/to/the/socket") {
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:1521:5
[01:44:50]   |
[01:44:50] 5 | use std::os::unix::net::UnixSeqpacketListener;
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] thread 'sys/unix/ext/net.rs - sys::unix::ext::net::UnixSeqpacketListener::bind (line 1519)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:44:50] 
[01:44:50] ---- sys/unix/ext/net.rs - sys::unix::ext::net::UnixSeqpacketListener::accept (line 1557) stdout ----
[01:44:50]  error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:1561:16
[01:44:50]   |
[01:44:50] 7 | let listener = UnixSeqpacketListener::bind("/path/to/the/socket").unwrap();
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:1559:5
[01:44:50]   |
[01:44:50] 5 | use std::os::unix::net::UnixSeqpacketListener;
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:1563:16
[01:44:50]   |
[01:44:50] 9 | match listener.accept() {
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] thread 'sys/unix/ext/net.rs - sys::unix::ext::net::UnixSeqpacketListener::accept (line 1557)' panicked at 'couldn't compile the test', librustdoc/test.rs:321:13
[01:44:50] 
[01:44:50] ---- sys/unix/ext/net.rs - sys::unix::ext::net::UnixSeqpacketListener::local_addr (line 1602) stdout ----
[01:44:50]  error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:1606:16
[01:44:50]   |
[01:44:50] 7 | let listener = UnixSeqpacketListener::bind("/path/to/the/socket").unwrap();
[01:44:50]   |
[01:44:50]   |
[01:44:50]   = help: add #![feature(unix_socket_seqpacket)] to the crate attributes to enable
[01:44:50] 
[01:44:50] error[E0658]: use of unstable library feature 'unix_socket_seqpacket'
[01:44:50]  --> sys/unix/ext/net.rs:1604:5
[01:44:50]   |
[01:44:50] 5 | use std::os::unix::net::UnixSeqpacketListener;
[01:44:50]   |
---
60840 ./src/llvm-emscripten/lib
56088 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/bin
55348 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release
53584 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental/syntax-284je5yviillk
53580 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental/syntax-284je5yviillk/s-f0mogyxocs-ik70nd-kr0695awam0u
53240 ./obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps
48604 ./obj/build/x86_64-unknown-linux-gnu/stage0/bin
47832 ./obj/build/x86_64-unknown-linux-gnu/stage0-std
46856 ./src/test
