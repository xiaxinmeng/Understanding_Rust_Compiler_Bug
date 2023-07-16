
$ rustc --pretty normal foo.rs | rustc -
error: internal compiler error: unexpected failure
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'No def'n found for syntax::ast::DefId{krate: 0u32, node: 4u32} in tcx.tcache', /home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/src/librustc/middle/ty.rs:3845

