console
$ rustc -Zmir-opt-level=0 b.rs && ./b
$ rustc -Zmir-opt-level=1 b.rs && ./b
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `A`,
 right: `B`', b.rs:18:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
