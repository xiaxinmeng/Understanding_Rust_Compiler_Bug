shell
cargo +stage1 run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/gh-107528`
WriteOwned::writev_all, calling writev with 2 items
self's type is gh_107528::TcpStream
TcpStream::write_v (delegate) with 2 buffers
TcpStream::write_v (concrete) with 2 buffers
thread 'main' panicked at 'not yet implemented', src/main.rs:36:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
