
type-resolution $ cargo version
cargo 1.32.0 (8610973aa 2019-01-02)
type-resolution $ cargo build
...
    Finished dev [unoptimized + debuginfo] target(s) in 3.36s
type-resolution $ rustup default 1.33.0
...
  1.33.0-x86_64-apple-darwin installed - rustc 1.33.0 (2aa4c46cf 2019-02-28)
type-resolution $ cargo version
cargo 1.33.0 (f099fe94b 2019-02-12)
type-resolution $ cargo build
...

error[E0308]: mismatched types
   --> src/main.rs:124:30
    |
124 |     build_server(|| |i: i32| make_server(i, MyServiceImpl {}))
    |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected trait `HttpService<Http2Transport, Response=bytes::bytes::Bytes, Response=bytes::bytes::Bytes, Request=bytes::bytes::Bytes, Request=bytes::bytes::Bytes, Handler=_> + std::marker::Send`, found trait `HttpService<_, Response=_, Request=_, Handler=MyServiceImpl> + std::marker::Send`
    |
    = note: expected type `std::result::Result<std::boxed::Box<(dyn HttpService<Http2Transport, Response=bytes::bytes::Bytes, Response=bytes::bytes::Bytes, Request=bytes::bytes::Bytes, Request=bytes::bytes::Bytes, Handler=_> + std::marker::Send + 'static)>, _>`
               found type `std::result::Result<std::boxed::Box<(dyn HttpService<_, Response=_, Request=_, Handler=MyServiceImpl> + std::marker::Send + 'static)>, _>`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: Could not compile `type-resolution`.
