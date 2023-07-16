rust
thread 'main' panicked at 'failed to determine underlying rustc version of Miri: CommandError { stdout: "", stderr: "/home/waffle/rust-b/build/x86_64-unknown-linux-gnu/stage1/bin/miri: error while loading shared libraries: librustc_driver-152a9025dac882e1.so: cannot open shared object file: No such file or directory\n" }', src/tools/miri/cargo-miri/src/util.rs:116:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
