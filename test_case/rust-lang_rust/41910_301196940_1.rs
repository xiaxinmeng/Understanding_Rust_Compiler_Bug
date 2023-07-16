
$ RUSTC=/usr/local/bin/rustc LD_LIBRARY_PATH=/usr/local/lib cargo test bench_bin_implicit
error[E0514]: found crate `log` compiled by an incompatible version of rustc
 --> /home/mike/dev/rust/cargo/src/cargo/lib.rs:5:14
  |
5 | #[macro_use] extern crate log;
  |              ^^^^^^^^^^^^^^^^^
  |
  = help: please recompile that crate using this compiler (rustc 1.19.0-nightly (f1140a331 2017-05-08))
  = note: crate `log` path #1: /home/mike/dev/rust/cargo/target/debug/deps/liblog-b8e0bbfdccaf7d7a.rlib compiled by "rustc 1.19.0-dev (e6cde9f2a 2017-05-11)"
