console
$ rustc +stage1 -Aunused a.rs -Zmir-opt-level=0 && ./a
$ rustc +stage1 -Aunused a.rs && ./a
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `2`', a.rs:12:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
