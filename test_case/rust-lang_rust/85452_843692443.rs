plain
      Memory: 14 GB
      Boot ROM Version: VMW71.00V.13989454.B64.1906190538
      Apple ROM Info: [MS_VM_CERT/SHA1/27d66596a61c48dd3dc7216fd715126e33f59ae7]Welcome to the Virtual Machine
      SMC Version (system): 2.8f0
      Serial Number (system): VMJLezMnzQxL

hw.ncpu: 3
hw.byteorder: 1234
hw.memsize: 15032385536
---
test src/time.rs - time::SystemTime::now (line 444) ... ok

failures:

---- src/os/./unix/net/addr.rs - os::doc::unix::net::addr::SocketAddr::as_abstract_namespace (line 204) stdout ----
error[E0599]: no function or associated item named `from_abstract_namespace` found for struct `std::os::unix::net::SocketAddr` in the current scope
  |
  |
8 |     let namespace_addr = SocketAddr::from_abstract_namespace(&namespace[..])?;


error[E0599]: no method named `as_abstract_namespace` found for struct `std::os::unix::net::SocketAddr` in the current scope
   |
   |
11 |     assert_eq!(local_addr.as_abstract_namespace(), Some(&namespace[..]));

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0599`.
For more information about this error, try `rustc --explain E0599`.
Couldn't compile the test.
---- src/os/./unix/net/addr.rs - os::doc::unix::net::addr::SocketAddr::from_abstract_namespace (line 258) stdout ----
error[E0599]: no function or associated item named `from_abstract_namespace` found for struct `std::os::unix::net::SocketAddr` in the current scope
  |
  |
7 |     let addr = SocketAddr::from_abstract_namespace(b"hidden")?;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
For more information about this error, try `rustc --explain E0599`.
Couldn't compile the test.
---- src/os/./unix/net/datagram.rs - os::doc::unix::net::datagram::UnixDatagram::bind_addr (line 119) stdout ----
error[E0599]: no function or associated item named `from_abstract_namespace` found for struct `std::os::unix::net::SocketAddr` in the current scope
  |
  |
7 |     let addr = SocketAddr::from_abstract_namespace(b"hidden")?; // Linux only

error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
For more information about this error, try `rustc --explain E0599`.
Couldn't compile the test.
---- src/os/./unix/net/datagram.rs - os::doc::unix::net::datagram::UnixDatagram::connect_addr (line 232) stdout ----
error[E0599]: no function or associated item named `from_abstract_namespace` found for struct `std::os::unix::net::SocketAddr` in the current scope
  |
  |
7 |     let addr = SocketAddr::from_abstract_namespace(b"hidden")?; // Linux only

error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
For more information about this error, try `rustc --explain E0599`.
Couldn't compile the test.
---- src/os/./unix/net/datagram.rs - os::doc::unix::net::datagram::UnixDatagram::send_to_addr (line 550) stdout ----
error[E0599]: no function or associated item named `from_abstract_namespace` found for struct `std::os::unix::net::SocketAddr` in the current scope
  |
  |
7 |     let addr = SocketAddr::from_abstract_namespace(b"hidden")?;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
For more information about this error, try `rustc --explain E0599`.
Couldn't compile the test.
---- src/os/./unix/net/listener.rs - os::doc::unix::net::listener::UnixListener::bind_addr (line 90) stdout ----
error[E0599]: no function or associated item named `from_abstract_namespace` found for struct `std::os::unix::net::SocketAddr` in the current scope
  |
  |
7 |     let addr = SocketAddr::from_abstract_namespace(b"namespace")?; // Linux only

error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
For more information about this error, try `rustc --explain E0599`.
Couldn't compile the test.
---- src/os/./unix/net/stream.rs - os::doc::unix::net::stream::UnixStream::connect_addr (line 113) stdout ----
error[E0599]: no function or associated item named `from_abstract_namespace` found for struct `std::os::unix::net::SocketAddr` in the current scope
  |
  |
7 |     let addr = SocketAddr::from_abstract_namespace(b"hidden")?; // Linux only

error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/Users/runner/work/rust/rust/build/x86_64-apple-darwin/stage0/bin/cargo" "test" "--target" "x86_64-apple-darwin" "-Zbinary-dep-depinfo" "-j" "3" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler compiler-builtins-c" "--manifest-path" "/Users/runner/work/rust/rust/library/test/Cargo.toml" "-p" "std" "--"


failed to run: /Users/runner/work/rust/rust/build/bootstrap/debug/bootstrap --stage 2 test
Build completed unsuccessfully in 1:47:51
