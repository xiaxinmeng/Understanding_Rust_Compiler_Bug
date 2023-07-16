
$ rustc foo.rs
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'assertion failed: did.krate != ast::LOCAL_CRATE', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/middle/ty.rs:5341
