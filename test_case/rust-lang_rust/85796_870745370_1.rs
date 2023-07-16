console
$ rustc +stage1 -O b.rs -Zmir-opt-level=3 && ./b
$ rustc +stage1 -O b.rs -Zmir-opt-level=4 && ./b
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `140732655603256`,
 right: `140732655603248`', b.rs:13:18
