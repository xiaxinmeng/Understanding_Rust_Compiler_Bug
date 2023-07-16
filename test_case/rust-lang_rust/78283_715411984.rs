
$ rustc -O -C codegen-units=1 test.rs && ./test

$ rustc -O -C codegen-units=1 test.rs --target x86_64-pc-windows-msvc && ./test
thread 'main' panicked at 'index out of bounds: the len is 0 but the index is 0', test.rs:21:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
